const fs = require('fs');
const d3 = require('d3-scale');
const d3Axis = require('d3-axis');
const d3Time = require('d3-time');
const d3TimeFormat = require('d3-time-format');

// Helper function to extract tick values from D3 axis
function getAxisTicks(scale, tickCount, isTimeScale = false) {
    const axis = d3Axis.axisBottom(scale);
    if (tickCount !== undefined) {
        axis.ticks(tickCount);
    }
    
    const ticks = scale.ticks ? scale.ticks(tickCount || 10) : scale.domain();
    const tickFormat = axis.tickFormat() || (d => d.toString());
    
    return ticks.map(tick => ({
        value: tick instanceof Date ? tick.toISOString() : tick,
        position: scale(tick),
        label: tickFormat(tick)
    }));
}

// Generate reference data
const referenceData = {
    linear: {
        normal: [],
        zeroSpan: [],
        singleValue: []
    },
    time: {
        seconds: [],
        minutes: [],
        hours: [],
        days: [],
        months: [],
        years: []
    }
};

// Linear scale tests
console.log('Generating linear scale reference data...');

// Normal domain
const linearNormal = d3.scaleLinear().domain([0, 10]).range([0, 100]);
referenceData.linear.normal = getAxisTicks(linearNormal, 5);

// Zero span domain
const linearZeroSpan = d3.scaleLinear().domain([0, 0]).range([0, 100]);
referenceData.linear.zeroSpan = getAxisTicks(linearZeroSpan, 5);

// Single value domain (same as zero span but with non-zero value)
const linearSingleValue = d3.scaleLinear().domain([5, 5]).range([10, 10]);
referenceData.linear.singleValue = getAxisTicks(linearSingleValue, 1);

// Time scale tests
console.log('Generating time scale reference data...');

// Different time spans
const timeRanges = [
    {
        name: 'seconds',
        start: new Date(2020, 0, 1, 0, 0, 0),
        end: new Date(2020, 0, 1, 0, 0, 4),
        ticks: 5
    },
    {
        name: 'minutes',
        start: new Date(2020, 0, 1, 0, 0, 0),
        end: new Date(2020, 0, 1, 0, 5, 0),
        ticks: 6
    },
    {
        name: 'hours',
        start: new Date(2020, 0, 1, 0, 0, 0),
        end: new Date(2020, 0, 1, 4, 0, 0),
        ticks: 5
    },
    {
        name: 'days',
        start: new Date(2020, 0, 1),
        end: new Date(2020, 0, 5),
        ticks: 5
    },
    {
        name: 'months',
        start: new Date(2020, 0, 1),
        end: new Date(2020, 4, 1),
        ticks: 5
    },
    {
        name: 'years',
        start: new Date(2020, 0, 1),
        end: new Date(2024, 0, 1),
        ticks: 5
    }
];

timeRanges.forEach(range => {
    console.log(`Generating ${range.name} time scale...`);
    const timeScale = d3.scaleTime().domain([range.start, range.end]).range([0, 100]);
    referenceData.time[range.name] = getAxisTicks(timeScale, range.ticks);
});

// Output the reference data
console.log('Writing reference data...');

// Write JSON files
fs.writeFileSync('linear_reference.json', JSON.stringify(referenceData.linear, null, 2));
fs.writeFileSync('time_reference.json', JSON.stringify(referenceData.time, null, 2));

// Also create a combined file for easy access
fs.writeFileSync('d3_axis_reference.json', JSON.stringify(referenceData, null, 2));

console.log('Reference data generated successfully!');
console.log('Files created:');
console.log('- linear_reference.json');
console.log('- time_reference.json');
console.log('- d3_axis_reference.json');

// Print a summary
console.log('\nSummary:');
console.log('Linear scales:');
console.log(`  Normal domain [0, 10]: ${referenceData.linear.normal.length} ticks`);
console.log(`  Zero span domain [0, 0]: ${referenceData.linear.zeroSpan.length} ticks`);
console.log(`  Single value domain [5, 5]: ${referenceData.linear.singleValue.length} ticks`);

console.log('\nTime scales:');
Object.entries(referenceData.time).forEach(([name, ticks]) => {
    console.log(`  ${name}: ${ticks.length} ticks`);
});

// Print sample data for verification
console.log('\nSample linear ticks (normal domain):');
referenceData.linear.normal.forEach((tick, i) => {
    console.log(`  ${i}: value=${tick.value}, position=${tick.position}, label="${tick.label}"`);
});

console.log('\nSample time ticks (seconds):');
referenceData.time.seconds.forEach((tick, i) => {
    console.log(`  ${i}: value=${tick.value}, position=${tick.position}, label="${tick.label}"`);
});
