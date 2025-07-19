# D3-Axis vs Rust Axis Renderers - Gap Analysis

This document analyzes the behavioral differences between the upstream D3-axis JavaScript reference (`tests/fixtures/node_modules/d3-axis/src/axis.js`) and the Rust axis renderer implementations.

## **1. Default Offset Logic (devicePixelRatio Awareness)**

### **JavaScript D3 Reference (axis.js:38)**
```javascript
offset = typeof window !== "undefined" && window.devicePixelRatio > 1 ? 0 : 0.5,
```

### **Rust Implementation (axis_impl.rs:20)**
```rust
offset: 0.5,
```

### **❌ DIVERGENCE:**
- **D3**: Uses devicePixelRatio-aware logic - sets `offset = 0` on high-DPI displays (`devicePixelRatio > 1`), `0.5` otherwise
- **Rust**: Always defaults to `0.5` regardless of display characteristics
- **Impact**: Rust may produce blurry lines on high-DPI displays where D3 would use crisp pixel-aligned rendering

---

## **2. Order/Timing of Selection/Enter/Update/Exit Operations**

### **JavaScript D3 Reference (axis.js:52-89)**
```javascript
// Selection phase
var selection = context.selection ? context.selection() : context,
    path = selection.selectAll(".domain").data([null]),
    tick = selection.selectAll(".tick").data(values, scale).order(),
    tickExit = tick.exit(),
    tickEnter = tick.enter().append("g").attr("class", "tick"),
    line = tick.select("line"),
    text = tick.select("text");

// Enter phase - merge operations
path = path.merge(path.enter().insert("path", ".tick")
    .attr("class", "domain")
    .attr("stroke", "currentColor"));

tick = tick.merge(tickEnter);
line = line.merge(tickEnter.append("line")...);
text = text.merge(tickEnter.append("text")...);

// Update/Exit phase with transitions
if (context !== selection) {
    path = path.transition(context);
    tick = tick.transition(context);
    line = line.transition(context);
    text = text.transition(context);
    
    tickExit = tickExit.transition(context)
        .attr("opacity", epsilon)...;
}

tickExit.remove(); // Exit happens here

// Final update phase
path.attr("d", ...);
tick.attr("opacity", 1)...;
```

### **Rust Implementation (all renderers)**
```rust
// Direct DOM manipulation - no selection/enter/update/exit pattern
let mut domain_line = selection.append("line");  // Direct append
for tick in &ticks {
    let mut tick_line = selection.append("line");  // Direct append
    let mut label = selection.append("text");      // Direct append
}
```

### **❌ MAJOR DIVERGENCE:**
- **D3**: Uses proper selection/enter/update/exit lifecycle with data binding, transitions, and element reuse
- **Rust**: Direct DOM manipulation without data binding or element lifecycle management
- **Impact**: 
  - No smooth transitions between axis updates
  - No element reuse (performance impact)
  - No automatic enter/update/exit handling for dynamic data
  - Elements accumulate on re-render instead of being properly managed

---

## **3. Path "Domain" Drawing Commands (Inner/Outer Size Math)**

### **JavaScript D3 Reference (axis.js:92-94)**
```javascript
path.attr("d", orient === left || orient === right
    ? (tickSizeOuter ? "M" + k * tickSizeOuter + "," + range0 + "H" + offset + "V" + range1 + "H" + k * tickSizeOuter : "M" + offset + "," + range0 + "V" + range1)
    : (tickSizeOuter ? "M" + range0 + "," + k * tickSizeOuter + "V" + offset + "H" + range1 + "V" + k * tickSizeOuter : "M" + range0 + "," + offset + "H" + range1));
```

**D3 creates SVG path with proper commands:**
- **With tickSizeOuter**: `M{outer},{range0}H{offset}V{range1}H{outer}` (creates outer ticks)
- **Without tickSizeOuter**: `M{offset},{range0}V{range1}` (simple line)

### **Rust Implementation (linear/time/log renderers)**
```rust
// Uses simple line elements instead of path
domain_line.attr("x1", &range0.to_string())
    .attr("x2", &range1.to_string()) 
    .attr("y1", "0")
    .attr("y2", "0");
```

### **❌ MAJOR DIVERGENCE:**
- **D3**: Uses SVG `<path>` element with proper path commands including outer tick integration
- **Rust**: Uses simple `<line>` elements without outer tick size consideration  
- **Impact**: 
  - Missing outer ticks at axis endpoints
  - Incorrect visual rendering compared to D3
  - No integration between domain line and tickSizeOuter parameter

---

## **4. Tick/Label Positioning and Anchoring**

### **JavaScript D3 Reference (axis.js:67-72, 96-105)**
```javascript
// Tick line positioning
line = line.merge(tickEnter.append("line")
    .attr("stroke", "currentColor")
    .attr(x + "2", k * tickSizeInner));

// Label positioning with proper anchoring
text = text.merge(tickEnter.append("text")
    .attr("fill", "currentColor")  
    .attr(x, k * spacing)
    .attr("dy", orient === top ? "0em" : orient === bottom ? "0.71em" : "0.32em"));

// Transform application
tick.attr("transform", function(d) { return transform(position(d) + offset); });
```

### **Rust Implementation (axis_renderable_linear.rs:180-225)**
```rust
// Tick positioning - manually calculated per orientation
match self.orientation {
    AxisOrientation::Bottom => {
        tick_line.attr("x1", &(tick.position + self.offset).to_string())
            .attr("x2", &(tick.position + self.offset).to_string())
            .attr("y1", "0")
            .attr("y2", &self.tick_size_inner.to_string());
        
        label.attr("x", &(tick.position + self.offset).to_string())
            .attr("y", &(k * spacing).to_string())
            .attr("text-anchor", "middle")
            .attr("dy", "0.71em");  // ✅ Matches D3
    }
    // ... other orientations
}
```

### **⚠️ PARTIAL DIVERGENCE:**
- **D3**: Uses generic transform-based positioning with orientation-agnostic logic
- **Rust**: Hard-coded positioning logic per orientation (more verbose but functionally equivalent)
- **✅ MATCHES**: `dy` values match D3 exactly ("0em", "0.71em", "0.32em")
- **✅ MATCHES**: Text anchoring logic matches D3 ("middle", "start", "end")
- **✅ MATCHES**: Spacing calculation matches D3

---

## **5. Half-Pixel Translation for Crispness**

### **JavaScript D3 Reference (axis.js:98)**
```javascript
tick.attr("transform", function(d) { return transform(position(d) + offset); });
```
Where `transform` is either `translateX` or `translateY` and includes the offset.

### **Rust Implementation (varies by renderer)**

#### **Linear Renderer (axis_renderable_linear.rs:10-33)**
```rust
// ✅ GOOD: Applies transform with offset
let final_transform = match &existing_transform {
    Some(existing) => format!("{} {}", existing, offset_transform),
    None => offset_transform
};
selection.attr("transform", &final_transform);

// ❌ ISSUE: Also adds offset to individual element positioning
.attr("x1", &(tick.position + self.offset).to_string())
```

#### **Time Renderer (axis_renderable_time.rs:21-41)**
```rust 
// ❌ ISSUE: Conditional offset application
let final_transform = match &existing_transform {
    Some(existing) => {
        if self.offset != 0.0 {  // ❌ Should always apply
            format!("{} {}", existing, offset_transform)
        } else {
            existing.clone()  // ❌ Skips offset application
        }
    }
    None => {
        if self.offset != 0.0 {  // ❌ Should always apply  
            offset_transform
        } else {
            String::new()  // ❌ No transform applied
        }
    }
};
```

### **❌ DIVERGENCES:**
1. **Time renderer**: Incorrect conditional offset logic - should always apply offset like D3
2. **Linear renderer**: Double offset application (transform + individual positioning)
3. **Default offset**: Should be devicePixelRatio-aware, not always 0.5

---

## **6. Element Order and Structure**

### **JavaScript D3 Reference**
```javascript
// Domain path inserted before ticks
path = path.merge(path.enter().insert("path", ".tick")
    .attr("class", "domain"));

// Tick groups with line and text children  
tickEnter = tick.enter().append("g").attr("class", "tick");
line = line.merge(tickEnter.append("line"));
text = text.merge(tickEnter.append("text"));
```

### **Rust Implementation**
```rust
// ❌ WRONG ORDER: Domain line appended after other elements
// Various elements appended directly without grouping structure
selection.append("line"); // tick lines
selection.append("text"); // tick labels  
selection.append("line"); // domain line (should be first)
```

### **❌ DIVERGENCE:**
- **D3**: Domain inserted before ticks, proper nesting with tick groups
- **Rust**: Flat structure, domain line appended last, no tick grouping

---

## **7. Attribute and Styling Consistency**

### **JavaScript D3 Reference (axis.js:60-61, 66-67, 69-71)**
```javascript
.attr("class", "domain")
.attr("stroke", "currentColor")

.attr("stroke", "currentColor")
.attr(x + "2", k * tickSizeInner)

.attr("fill", "currentColor")
```

### **Rust Implementation**
```rust
// ✅ MATCHES: Uses "currentColor" 
.attr("stroke", "currentColor")
.attr("fill", "currentColor")

// ❌ INCONSISTENT: Some renderers use hardcoded colors
.attr("stroke", "black")  // Should be "currentColor"
```

### **⚠️ MIXED CONFORMANCE:**
- Linear/Time renderers: ✅ Use "currentColor" correctly
- Log/Point renderers: ❌ Use hardcoded "black" color

---

## **8. Transition and Animation Support**

### **JavaScript D3 Reference (axis.js:74-87)**
```javascript
if (context !== selection) {
    path = path.transition(context);
    tick = tick.transition(context); 
    line = line.transition(context);
    text = text.transition(context);

    tickExit = tickExit.transition(context)
        .attr("opacity", epsilon)...;
        
    tickEnter
        .attr("opacity", epsilon)...;
}
```

### **Rust Implementation**
```rust
// ❌ NO TRANSITION SUPPORT
// Direct attribute setting with no animation capabilities
```

### **❌ COMPLETE DIVERGENCE:**
- **D3**: Full transition and animation support for smooth axis updates
- **Rust**: No transition support - immediate DOM updates only

---

## **Summary of Critical Divergences**

### **Priority 1 - Critical Issues:**
1. **Default offset logic**: Missing devicePixelRatio awareness
2. **Selection lifecycle**: No enter/update/exit pattern - direct DOM manipulation
3. **Domain path rendering**: Using `<line>` instead of proper SVG path with outer ticks
4. **Element ordering**: Domain should be inserted before ticks

### **Priority 2 - Important Issues:**  
5. **Time renderer offset**: Conditional offset application logic is incorrect
6. **Transition support**: Complete absence of animation capabilities
7. **Element structure**: Flat DOM instead of proper tick grouping

### **Priority 3 - Consistency Issues:**
8. **Color attributes**: Some renderers use hardcoded colors instead of "currentColor"
9. **Transform handling**: Inconsistent offset application patterns

### **Areas that Match D3:**
- ✅ Basic positioning calculations
- ✅ Text anchoring and `dy` attributes  
- ✅ Spacing calculations
- ✅ Orientation-specific logic

This analysis provides a roadmap for bringing the Rust axis renderers into full compliance with D3 behavior.
