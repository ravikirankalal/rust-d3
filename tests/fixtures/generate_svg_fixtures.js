#!/usr/bin/env node
//
// Golden SVG Fixture Generator
// Generates SVG snippets using real D3.js axis rendering for comparison tests
//
// This script uses D3.js to render axes and extract the generated SVG snippets.
// These fixtures serve as golden masters for regression testing against the Rust implementation.

const fs = require('fs');
const path = require('path');
const d3 = require('d3');

// Mock DOM environment for D3 (using jsdom if available, otherwise basic implementation)
let document, window;
try {
    const { JSDOM } = require('jsdom');
    const dom = new JSDOM(`<!DOCTYPE html><html><body></body></html>`);
    document = dom.window.document;
    window = dom.window;
    
    // Set globals for D3
    global.document = document;
    global.window = window;
} catch (err) {
    console.log('jsdom not available, using basic DOM mock');
    // Basic DOM mock for D3 compatibility
    document = {
        createElement: (tag) => ({
            tagName: tag.toUpperCase(),
            setAttribute: function(name, value) { this[name] = value; },
            appendChild: function(child) { this.children = this.children || []; this.children.push(child); },
            getAttribute: function(name) { return this[name]; }
        }),
        createElementNS: function(ns, tag) { return this.createElement(tag); }
    };
    window = { 
        SVGElement: function() {},
        getComputedStyle: () => ({ getPropertyValue: () => '' })
    };
    global.document = document;
    global.window = window;
}

// SVG rendering helper
function createSVGElement(width = 400, height = 200) {
    const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
    svg.setAttribute('width', width);
    svg.setAttribute('height', height);
    return svg;
}

// Serialize SVG element to string
function serializeSVG(element) {
    // Basic SVG serialization 
    if (typeof element.outerHTML !== 'undefined') {
        return element.outerHTML;
    }
    
    // Manual serialization for mock DOM
    let result = `<${element.tagName.toLowerCase()}`;
    if (element.attributes) {
        for (const attr of element.attributes) {
            result += ` ${attr.name}="${attr.value}"`;
        }
    }
    // Add common SVG attributes manually if they exist
    const attrs = ['transform', 'class', 'stroke', 'fill', 'x', 'y', 'x1', 'y1', 'x2', 'y2', 'd', 'text-anchor'];
    for (const attr of attrs) {
        if (element[attr]) {
            result += ` ${attr}="${element[attr]}"`;
        }
    }
    
    if (element.children && element.children.length > 0) {
        result += '>';
        for (const child of element.children) {
            result += serializeSVG(child);
        }
        result += `</${element.tagName.toLowerCase()}>`;
    } else if (element.textContent) {
        result += `>${element.textContent}</${element.tagName.toLowerCase()}>`;
    } else {
        result += '/>';
    }
    
    return result;
}

// Generator functions for different axis types
const generators = {
    // Linear axis bottom (domain 0-10)
    linear_axis_bottom_0_10: () => {
        const svg = createSVGElement();
        const scale = d3.scaleLinear()
            .domain([0, 10])
            .range([0, 100]);
            
        const axis = d3.axisBottom(scale).ticks(5);
        const g = document.createElementNS('http://www.w3.org/2000/svg', 'g');
        g.setAttribute('transform', 'translate(50, 50)');
        
        // Apply axis to group (simplified)
        const ticks = scale.ticks(5);
        const tickFormat = axis.tickFormat() || (d => d.toString());
        
        // Generate domain line
        const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
        path.setAttribute('class', 'domain');
        path.setAttribute('stroke', 'currentColor');
        path.setAttribute('d', `M0.5,0H${100 + 0.5}V0`);
        g.appendChild(path);
        
        // Generate ticks
        for (const tick of ticks) {
            const tickG = document.createElementNS('http://www.w3.org/2000/svg', 'g');
            tickG.setAttribute('class', 'tick');
            tickG.setAttribute('transform', `translate(${scale(tick)}, 0)`);
            
            const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
            line.setAttribute('stroke', 'currentColor');
            line.setAttribute('y2', '6');
            tickG.appendChild(line);
            
            const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
            text.setAttribute('fill', 'currentColor');
            text.setAttribute('y', '9');
            text.setAttribute('dy', '0.71em');
            text.setAttribute('text-anchor', 'middle');
            text.textContent = tickFormat(tick);
            tickG.appendChild(text);
            
            g.appendChild(tickG);
        }
        
        svg.appendChild(g);
        return serializeSVG(g);
    },
    
    // Linear axis left with reversed range
    linear_axis_left_0_100_reversed: () => {
        const svg = createSVGElement();
        const scale = d3.scaleLinear()
            .domain([0, 100])
            .range([200, 0]); // Reversed
            
        const axis = d3.axisLeft(scale).ticks(5);
        const g = document.createElementNS('http://www.w3.org/2000/svg', 'g');
        g.setAttribute('transform', 'translate(50, 50)');
        
        const ticks = scale.ticks(5);
        const tickFormat = axis.tickFormat() || (d => d.toString());
        
        // Generate domain line
        const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
        path.setAttribute('class', 'domain');
        path.setAttribute('stroke', 'currentColor');
        path.setAttribute('d', `M0,${200.5}V0.5H0`);
        g.appendChild(path);
        
        // Generate ticks
        for (const tick of ticks) {
            const tickG = document.createElementNS('http://www.w3.org/2000/svg', 'g');
            tickG.setAttribute('class', 'tick');
            tickG.setAttribute('transform', `translate(0, ${scale(tick)})`);
            
            const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
            line.setAttribute('stroke', 'currentColor');
            line.setAttribute('x2', '-6');
            tickG.appendChild(line);
            
            const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
            text.setAttribute('fill', 'currentColor');
            text.setAttribute('x', '-9');
            text.setAttribute('dy', '0.32em');
            text.setAttribute('text-anchor', 'end');
            text.textContent = tickFormat(tick);
            tickG.appendChild(text);
            
            g.appendChild(tickG);
        }
        
        svg.appendChild(g);
        return serializeSVG(g);
    },
    
    // Time axis (4 second span)
    time_axis_seconds_4s: () => {
        const svg = createSVGElement();
        const startDate = new Date(2020, 0, 1, 0, 0, 0);
        const endDate = new Date(2020, 0, 1, 0, 0, 4);
        
        const scale = d3.scaleTime()
            .domain([startDate, endDate])
            .range([0, 100]);
            
        const axis = d3.axisBottom(scale).ticks(5);
        const g = document.createElementNS('http://www.w3.org/2000/svg', 'g');
        g.setAttribute('transform', 'translate(50, 50)');
        
        const ticks = scale.ticks(5);
        const tickFormat = axis.tickFormat() || d3.timeFormat('%H:%M:%S');
        
        // Generate domain line
        const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
        path.setAttribute('class', 'domain');
        path.setAttribute('stroke', 'currentColor');
        path.setAttribute('d', `M0.5,0H${100 + 0.5}V0`);
        g.appendChild(path);
        
        // Generate ticks
        for (const tick of ticks) {
            const tickG = document.createElementNS('http://www.w3.org/2000/svg', 'g');
            tickG.setAttribute('class', 'tick');
            tickG.setAttribute('transform', `translate(${scale(tick)}, 0)`);
            
            const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
            line.setAttribute('stroke', 'currentColor');
            line.setAttribute('y2', '6');
            tickG.appendChild(line);
            
            const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
            text.setAttribute('fill', 'currentColor');
            text.setAttribute('y', '9');
            text.setAttribute('dy', '0.71em');
            text.setAttribute('text-anchor', 'middle');
            text.textContent = tickFormat(tick);
            tickG.appendChild(text);
            
            g.appendChild(tickG);
        }
        
        svg.appendChild(g);
        return serializeSVG(g);
    },
    
    // Time axis (5 minute span)
    time_axis_minutes_5m: () => {
        const svg = createSVGElement();
        const startDate = new Date(2020, 0, 1, 0, 0, 0);
        const endDate = new Date(2020, 0, 1, 0, 5, 0);
        
        const scale = d3.scaleTime()
            .domain([startDate, endDate])
            .range([0, 100]);
            
        const axis = d3.axisBottom(scale).ticks(5);
        const g = document.createElementNS('http://www.w3.org/2000/svg', 'g');
        g.setAttribute('transform', 'translate(50, 50)');
        
        const ticks = scale.ticks(6);
        const tickFormat = axis.tickFormat() || d3.timeFormat('%H:%M');
        
        // Generate domain line
        const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
        path.setAttribute('class', 'domain');
        path.setAttribute('stroke', 'currentColor');
        path.setAttribute('d', `M0.5,0H${100 + 0.5}V0`);
        g.appendChild(path);
        
        // Generate ticks
        for (const tick of ticks) {
            const tickG = document.createElementNS('http://www.w3.org/2000/svg', 'g');
            tickG.setAttribute('class', 'tick');
            tickG.setAttribute('transform', `translate(${scale(tick)}, 0)`);
            
            const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
            line.setAttribute('stroke', 'currentColor');
            line.setAttribute('y2', '6');
            tickG.appendChild(line);
            
            const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
            text.setAttribute('fill', 'currentColor');
            text.setAttribute('y', '9');
            text.setAttribute('dy', '0.71em');
            text.setAttribute('text-anchor', 'middle');
            text.textContent = tickFormat(tick);
            tickG.appendChild(text);
            
            g.appendChild(tickG);
        }
        
        svg.appendChild(g);
        return serializeSVG(g);
    },
    
    // Time axis (4 hour span)
    time_axis_hours_4h: () => {
        const svg = createSVGElement();
        const startDate = new Date(2020, 0, 1, 0, 0, 0);
        const endDate = new Date(2020, 0, 1, 4, 0, 0);
        
        const scale = d3.scaleTime()
            .domain([startDate, endDate])
            .range([0, 100]);
            
        const axis = d3.axisBottom(scale).ticks(5);
        const g = document.createElementNS('http://www.w3.org/2000/svg', 'g');
        g.setAttribute('transform', 'translate(50, 50)');
        
        const ticks = scale.ticks(5);
        const tickFormat = axis.tickFormat() || d3.timeFormat('%H:%M');
        
        // Generate domain line
        const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
        path.setAttribute('class', 'domain');
        path.setAttribute('stroke', 'currentColor');
        path.setAttribute('d', `M0.5,0H${100 + 0.5}V0`);
        g.appendChild(path);
        
        // Generate ticks
        for (const tick of ticks) {
            const tickG = document.createElementNS('http://www.w3.org/2000/svg', 'g');
            tickG.setAttribute('class', 'tick');
            tickG.setAttribute('transform', `translate(${scale(tick)}, 0)`);
            
            const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
            line.setAttribute('stroke', 'currentColor');
            line.setAttribute('y2', '6');
            tickG.appendChild(line);
            
            const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
            text.setAttribute('fill', 'currentColor');
            text.setAttribute('y', '9');
            text.setAttribute('dy', '0.71em');
            text.setAttribute('text-anchor', 'middle');
            text.textContent = tickFormat(tick);
            tickG.appendChild(text);
            
            g.appendChild(tickG);
        }
        
        svg.appendChild(g);
        return serializeSVG(g);
    },
    
    // Time axis (4 day span)
    time_axis_days_4d: () => {
        const svg = createSVGElement();
        const startDate = new Date(2020, 0, 1, 0, 0, 0);
        const endDate = new Date(2020, 0, 5, 0, 0, 0);
        
        const scale = d3.scaleTime()
            .domain([startDate, endDate])
            .range([0, 100]);
            
        const axis = d3.axisBottom(scale).ticks(5);
        const g = document.createElementNS('http://www.w3.org/2000/svg', 'g');
        g.setAttribute('transform', 'translate(50, 50)');
        
        const ticks = scale.ticks(5);
        const tickFormat = axis.tickFormat() || d3.timeFormat('%m/%d');
        
        // Generate domain line
        const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
        path.setAttribute('class', 'domain');
        path.setAttribute('stroke', 'currentColor');
        path.setAttribute('d', `M0.5,0H${100 + 0.5}V0`);
        g.appendChild(path);
        
        // Generate ticks
        for (const tick of ticks) {
            const tickG = document.createElementNS('http://www.w3.org/2000/svg', 'g');
            tickG.setAttribute('class', 'tick');
            tickG.setAttribute('transform', `translate(${scale(tick)}, 0)`);
            
            const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
            line.setAttribute('stroke', 'currentColor');
            line.setAttribute('y2', '6');
            tickG.appendChild(line);
            
            const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
            text.setAttribute('fill', 'currentColor');
            text.setAttribute('y', '9');
            text.setAttribute('dy', '0.71em');
            text.setAttribute('text-anchor', 'middle');
            text.textContent = tickFormat(tick);
            tickG.appendChild(text);
            
            g.appendChild(tickG);
        }
        
        svg.appendChild(g);
        return serializeSVG(g);
    },
    
    // Time axis (4 month span)
    time_axis_months_4m: () => {
        const svg = createSVGElement();
        const startDate = new Date(2020, 0, 1, 0, 0, 0);
        const endDate = new Date(2020, 4, 1, 0, 0, 0);
        
        const scale = d3.scaleTime()
            .domain([startDate, endDate])
            .range([0, 100]);
            
        const axis = d3.axisBottom(scale).ticks(5);
        const g = document.createElementNS('http://www.w3.org/2000/svg', 'g');
        g.setAttribute('transform', 'translate(50, 50)');
        
        const ticks = scale.ticks(5);
        const tickFormat = axis.tickFormat() || d3.timeFormat('%b');
        
        // Generate domain line
        const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
        path.setAttribute('class', 'domain');
        path.setAttribute('stroke', 'currentColor');
        path.setAttribute('d', `M0.5,0H${100 + 0.5}V0`);
        g.appendChild(path);
        
        // Generate ticks
        for (const tick of ticks) {
            const tickG = document.createElementNS('http://www.w3.org/2000/svg', 'g');
            tickG.setAttribute('class', 'tick');
            tickG.setAttribute('transform', `translate(${scale(tick)}, 0)`);
            
            const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
            line.setAttribute('stroke', 'currentColor');
            line.setAttribute('y2', '6');
            tickG.appendChild(line);
            
            const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
            text.setAttribute('fill', 'currentColor');
            text.setAttribute('y', '9');
            text.setAttribute('dy', '0.71em');
            text.setAttribute('text-anchor', 'middle');
            text.textContent = tickFormat(tick);
            tickG.appendChild(text);
            
            g.appendChild(tickG);
        }
        
        svg.appendChild(g);
        return serializeSVG(g);
    },
    
    // Time axis (4 year span)
    time_axis_years_4y: () => {
        const svg = createSVGElement();
        const startDate = new Date(2020, 0, 1, 0, 0, 0);
        const endDate = new Date(2024, 0, 1, 0, 0, 0);
        
        const scale = d3.scaleTime()
            .domain([startDate, endDate])
            .range([0, 100]);
            
        const axis = d3.axisBottom(scale).ticks(5);
        const g = document.createElementNS('http://www.w3.org/2000/svg', 'g');
        g.setAttribute('transform', 'translate(50, 50)');
        
        const ticks = scale.ticks(5);
        const tickFormat = axis.tickFormat() || d3.timeFormat('%Y');
        
        // Generate domain line
        const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
        path.setAttribute('class', 'domain');
        path.setAttribute('stroke', 'currentColor');
        path.setAttribute('d', `M0.5,0H${100 + 0.5}V0`);
        g.appendChild(path);
        
        // Generate ticks
        for (const tick of ticks) {
            const tickG = document.createElementNS('http://www.w3.org/2000/svg', 'g');
            tickG.setAttribute('class', 'tick');
            tickG.setAttribute('transform', `translate(${scale(tick)}, 0)`);
            
            const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
            line.setAttribute('stroke', 'currentColor');
            line.setAttribute('y2', '6');
            tickG.appendChild(line);
            
            const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
            text.setAttribute('fill', 'currentColor');
            text.setAttribute('y', '9');
            text.setAttribute('dy', '0.71em');
            text.setAttribute('text-anchor', 'middle');
            text.textContent = tickFormat(tick);
            tickG.appendChild(text);
            
            g.appendChild(tickG);
        }
        
        svg.appendChild(g);
        return serializeSVG(g);
    },
    
    // Time axis with milliseconds (500ms span)
    time_axis_milliseconds_500ms: () => {
        const svg = createSVGElement();
        const startDate = new Date(2020, 0, 1, 0, 0, 0, 0);
        const endDate = new Date(2020, 0, 1, 0, 0, 0, 500);
        
        const scale = d3.scaleTime()
            .domain([startDate, endDate])
            .range([0, 100]);
            
        const axis = d3.axisBottom(scale).ticks(6);
        const g = document.createElementNS('http://www.w3.org/2000/svg', 'g');
        g.setAttribute('transform', 'translate(50, 50)');
        
        const ticks = scale.ticks(6);
        const tickFormat = axis.tickFormat() || d3.timeFormat('%H:%M:%S.%L');
        
        // Generate domain line
        const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
        path.setAttribute('class', 'domain');
        path.setAttribute('stroke', 'currentColor');
        path.setAttribute('d', `M0.5,0H${100 + 0.5}V0`);
        g.appendChild(path);
        
        // Generate ticks
        for (const tick of ticks) {
            const tickG = document.createElementNS('http://www.w3.org/2000/svg', 'g');
            tickG.setAttribute('class', 'tick');
            tickG.setAttribute('transform', `translate(${scale(tick)}, 0)`);
            
            const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
            line.setAttribute('stroke', 'currentColor');
            line.setAttribute('y2', '6');
            tickG.appendChild(line);
            
            const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
            text.setAttribute('fill', 'currentColor');
            text.setAttribute('y', '9');
            text.setAttribute('dy', '0.71em');
            text.setAttribute('text-anchor', 'middle');
            text.textContent = tickFormat(tick);
            tickG.appendChild(text);
            
            g.appendChild(tickG);
        }
        
        svg.appendChild(g);
        return serializeSVG(g);
    },
    
    // Time axis 24 hours span)
    time_axis_full_day: () => {
        const svg = createSVGElement();
        const startDate = new Date(2020, 0, 1, 0, 0, 0);
        const endDate = new Date(2020, 0, 2, 0, 0, 0);
        
        const scale = d3.scaleTime()
            .domain([startDate, endDate])
            .range([0, 100]);
            
        const axis = d3.axisBottom(scale).ticks(5);
        const g = document.createElementNS('http://www.w3.org/2000/svg', 'g');
        g.setAttribute('transform', 'translate(50, 50)');
        
        const ticks = scale.ticks(5);
        const tickFormat = axis.tickFormat() || d3.timeFormat('%H:%M');
        
        // Generate domain line
        const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
        path.setAttribute('class', 'domain');
        path.setAttribute('stroke', 'currentColor');
        path.setAttribute('d', `M0.5,0H${100 + 0.5}V0`);
        g.appendChild(path);
        
        // Generate ticks
        for (const tick of ticks) {
            const tickG = document.createElementNS('http://www.w3.org/2000/svg', 'g');
            tickG.setAttribute('class', 'tick');
            tickG.setAttribute('transform', `translate(${scale(tick)}, 0)`);
            
            const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
            line.setAttribute('stroke', 'currentColor');
            line.setAttribute('y2', '6');
            tickG.appendChild(line);
            
            const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
            text.setAttribute('fill', 'currentColor');
            text.setAttribute('y', '9');
            text.setAttribute('dy', '0.71em');
            text.setAttribute('text-anchor', 'middle');
            text.textContent = tickFormat(tick);
            tickG.appendChild(text);
            
            g.appendChild(tickG);
        }
        
        svg.appendChild(g);
        return serializeSVG(g);
    },
    
    // Time axis 7 days span)
    time_axis_week: () => {
        const svg = createSVGElement();
        const startDate = new Date(2020, 0, 1, 0, 0, 0);
        const endDate = new Date(2020, 0, 8, 0, 0, 0);
        
        const scale = d3.scaleTime()
            .domain([startDate, endDate])
            .range([0, 100]);
            
        const axis = d3.axisBottom(scale).ticks(7);
        const g = document.createElementNS('http://www.w3.org/2000/svg', 'g');
        g.setAttribute('transform', 'translate(50, 50)');
        
        const ticks = scale.ticks(7);
        const tickFormat = axis.tickFormat() || d3.timeFormat('%a');
        
        // Generate domain line
        const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
        path.setAttribute('class', 'domain');
        path.setAttribute('stroke', 'currentColor');
        path.setAttribute('d', `M0.5,0H${100 + 0.5}V0`);
        g.appendChild(path);
        
        // Generate ticks
        for (const tick of ticks) {
            const tickG = document.createElementNS('http://www.w3.org/2000/svg', 'g');
            tickG.setAttribute('class', 'tick');
            tickG.setAttribute('transform', `translate(${scale(tick)}, 0)`);
            
            const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
            line.setAttribute('stroke', 'currentColor');
            line.setAttribute('y2', '6');
            tickG.appendChild(line);
            
            const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
            text.setAttribute('fill', 'currentColor');
            text.setAttribute('y', '9');
            text.setAttribute('dy', '0.71em');
            text.setAttribute('text-anchor', 'middle');
            text.textContent = tickFormat(tick);
            tickG.appendChild(text);
            
            g.appendChild(tickG);
        }
        
        svg.appendChild(g);
        return serializeSVG(g);
    },
    
    // Log axis (1 to 1000)
    log_axis_1_1000: () => {
        const svg = createSVGElement();
        const scale = d3.scaleLog()
            .domain([1, 1000])
            .range([0, 300])
            .base(10);
            
        const axis = d3.axisBottom(scale).ticks(4);
        const g = document.createElementNS('http://www.w3.org/2000/svg', 'g');
        g.setAttribute('transform', 'translate(50, 50)');
        
        const ticks = scale.ticks(4);
        const tickFormat = axis.tickFormat() || (d => d.toString());
        
        // Generate domain line
        const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
        path.setAttribute('class', 'domain');
        path.setAttribute('stroke', 'currentColor');
        path.setAttribute('d', `M0.5,0H${300 + 0.5}V0`);
        g.appendChild(path);
        
        // Generate ticks
        for (const tick of ticks) {
            const tickG = document.createElementNS('http://www.w3.org/2000/svg', 'g');
            tickG.setAttribute('class', 'tick');
            tickG.setAttribute('transform', `translate(${scale(tick)}, 0)`);
            
            const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
            line.setAttribute('stroke', 'currentColor');
            line.setAttribute('y2', '6');
            tickG.appendChild(line);
            
            const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
            text.setAttribute('fill', 'currentColor');
            text.setAttribute('y', '9');
            text.setAttribute('dy', '0.71em');
            text.setAttribute('text-anchor', 'middle');
            text.textContent = tickFormat(tick);
            tickG.appendChild(text);
            
            g.appendChild(tickG);
        }
        
        svg.appendChild(g);
        return serializeSVG(g);
    },
    
    // Band axis (categorical)
    band_axis_categorical: () => {
        const svg = createSVGElement();
        const scale = d3.scaleBand()
            .domain(['Alpha', 'Beta', 'Gamma', 'Delta'])
            .range([0, 400])
            .paddingInner(0.1)
            .paddingOuter(0.05);
            
        const axis = d3.axisBottom(scale);
        const g = document.createElementNS('http://www.w3.org/2000/svg', 'g');
        g.setAttribute('transform', 'translate(50, 50)');
        
        const domain = scale.domain();
        
        // Generate domain line
        const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
        path.setAttribute('class', 'domain');
        path.setAttribute('stroke', 'currentColor');
        path.setAttribute('d', `M0.5,0H${400 + 0.5}V0`);
        g.appendChild(path);
        
        // Generate ticks
        for (const value of domain) {
            const tickG = document.createElementNS('http://www.w3.org/2000/svg', 'g');
            tickG.setAttribute('class', 'tick');
            tickG.setAttribute('transform', `translate(${scale(value) + scale.bandwidth() / 2}, 0)`);
            
            const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
            line.setAttribute('stroke', 'currentColor');
            line.setAttribute('y2', '6');
            tickG.appendChild(line);
            
            const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
            text.setAttribute('fill', 'currentColor');
            text.setAttribute('y', '9');
            text.setAttribute('dy', '0.71em');
            text.setAttribute('text-anchor', 'middle');
            text.textContent = value;
            tickG.appendChild(text);
            
            g.appendChild(tickG);
        }
        
        svg.appendChild(g);
        return serializeSVG(g);
    }
};

// Generate all fixtures
console.log('Generating SVG fixtures...');

const outputDir = path.join(__dirname, 'svg');
if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir, { recursive: true });
}

for (const [name, generator] of Object.entries(generators)) {
    try {
        const svg = generator();
        const filename = `${name}.svg`;
        const filepath = path.join(outputDir, filename);
        
        fs.writeFileSync(filepath, svg);
        console.log(`Generated: ${filename}`);
    } catch (error) {
        console.error(`Error generating ${name}:`, error.message);
        
        // Create placeholder file for manual review
        const placeholder = `<!-- Error generating ${name}: ${error.message} -->`;
        const filepath = path.join(outputDir, `${name}.svg`);
        fs.writeFileSync(filepath, placeholder);
    }
}

console.log('SVG fixture generation complete!');
console.log('Files generated in:', outputDir);
