import Chart, { ChartConfiguration } from 'chart.js';
import { round } from 'lodash';
import { h, app } from 'hyperapp';
import * as core from 'f1-laps-js-bridge';


core.initialise({ updateInterval: 30, storagePath: '../../_data-storage' });

const state = {};
const actions = {};

const view = () => {
    return (
        <div id="container" onCreate={setTimeout(setupApp, 1000)}>
            <div class="log">
                <h3 class="lap-counter">Lap:</h3>
                <h3 class="time">Time:</h3>
                <div>
                    <h3>Stats</h3>
                    <ul class="stats">
                        <li class="fps"></li>
                        <li class="total-points"></li>
                        <li class="displayed-points"></li>
                        <li class="speed-points"></li>
                        <li class="throttle-points"></li>
                        <li class="brake-points"></li>
                        <li class="gear-points"></li>
                        <li class="steering-points"></li>
                        <li class="compressed-points"></li>
                    </ul>
                </div>
            </div>
            <canvas width="1200" height="200" id="speed-plot"></canvas>
            <canvas width="1200" height="200" id="throttle-plot"></canvas>
            <canvas width="1200" height="200" id="brake-plot"></canvas>
            <canvas width="1200" height="200" id="gear-plot"></canvas>
            <canvas width="1200" height="200" id="compressed-plot"></canvas>
            <canvas width="1200" height="200" id="steering-plot"></canvas>
        </div>
    );
};

app(state, actions, view, document.querySelector('#app'));

function setupApp() {
    let lapCounter = document.querySelector('.lap-counter');
    let timeElem = document.querySelector('.time');
    let fpsElem = document.querySelector('.fps');
    let displayedPointsElem = document.querySelector('.displayed-points');
    let compressedPointsElem = document.querySelector('.compressed-points');
    let totalPointsElem = document.querySelector('.total-points');
    let speedPointsElem = document.querySelector('.speed-points');
    let throttlePointsElem = document.querySelector('.throttle-points');
    let brakePointsElem = document.querySelector('.brake-points');
    let gearPointsElem = document.querySelector('.gear-points');
    let steeringPointsElem = document.querySelector('.steering-points');

    let speed = new Array<Point>();
    let throttle = new Array<Point>();
    let compressed = new Array<Point>();
    let brake = new Array<Point>();
    let gear = new Array<Point>();
    let steering = new Array<Point>();
    let time = new Array<number>();
    let lap = -1;

    function resetTraces() {
        speed = new Array<Point>();
        throttle = new Array<Point>();
        compressed = new Array<Point>();
        brake = new Array<Point>();
        gear = new Array<Point>();
        steering = new Array<Point>();
        time = new Array<number>();
    }

    const TIME_RANGE = 100;
    let totalDataPoints = 0;

    function subtract(a: Point, b: Point) {
        return {
            x: a.x - b.x,
            y: a.y - b.y
        };
    }

    function determinant(u: Point, v: Point) {
        return u.x * v.y - u.y * v.x;
    }

    function magnitude(u: Point) {
        return Math.sqrt(u.x ** 2 + u.y ** 2);
    }

    function removeHistory(arr: Array<Point | number>, currentTime: number) {
        const firstVisible = arr.findIndex(a => {
            if (typeof a === 'number') {
                return a > currentTime - TIME_RANGE - DELAY_SECONDS;
            } else {
                return a.x > currentTime - TIME_RANGE - DELAY_SECONDS
            }
        });

        if (firstVisible !== -1) {
            arr.splice(0, firstVisible - 1);
        }
    }

    function updateData(allData: Array<Point>, newPoint: Point, compressionTolerance = 0.001) {
        if (allData.length > 2) {
            const a = allData[allData.length - 2];
            const b = allData[allData.length - 1];
            const c = newPoint;

            const ab = subtract(b, a);
            const ac = subtract(c, a);

            const isColinear = Math.abs(determinant(ab, ac)) / magnitude(ac) < 0.001;
            if (isColinear) {
                allData[allData.length - 1] = c;
            } else {
                allData.push(c);
            }
        } else {
            allData.push(newPoint);
        }
    }

    core.liveData.register(data => {
        anyDataReceived = true;

        if (data.currentLap === 0 && lap > 0) {
            resetTraces();
        } else {
            removeHistory(speed, data.sessionTime);
            removeHistory(throttle, data.sessionTime);
            removeHistory(compressed, data.sessionTime);
            removeHistory(brake, data.sessionTime);
            removeHistory(gear, data.sessionTime);
            removeHistory(steering, data.sessionTime);
            removeHistory(time, data.sessionTime);

            updateData(speed, {
                x: data.sessionTime,
                y: data.currentSpeed
            });
            updateData(throttle, {
                x: data.sessionTime,
                y: data.throttle
            });
            updateData(compressed, { x: data.sessionTime, y: data.currentSpeed });


            updateData(brake, {
                x: data.sessionTime,
                y: data.brake
            });
            updateData(gear, {
                x: data.sessionTime,
                y: data.currentGear
            });
            updateData(steering, {
                x: data.sessionTime,
                y: data.steer
            });
            time.push(data.sessionTime);

            totalDataPoints++;
        }

        if (data.currentLap !== lap) {
            lap = data.currentLap;
            lapCounter.innerHTML = 'Lap: ' + lap;
        }
    });

    let speedPlot: Plot;
    let throttlePlot: Plot;
    let compressedPlot: Plot;
    let brakePlot: Plot;
    let gearPlot: Plot;
    let steeringPlot: Plot;
    let anyDataReceived = false;

    const DATA_UPDATE_INTERVAL = 1000;
    const DELAY_SECONDS = DATA_UPDATE_INTERVAL / 1000;
    const SCALE_UPDATE_INTERVAL = 75;
    const FPS_UPDATE_INTERVAL = 500;
    let lastDataUpdateTime = -DATA_UPDATE_INTERVAL;
    let lastScaleUpdateTime = -SCALE_UPDATE_INTERVAL;
    let lastFPSUpdateTime = -FPS_UPDATE_INTERVAL;
    let framesSinceUpdate = 0;

    function updatePlotData(plot: Chart, data: Array<Point>) {
        plot.data.datasets[0].data = data.slice();
        plot.update();
    }

    function updatePlotScale(plot: Plot, currentTime: number) {
        plot.options.scales.xAxes[0].ticks.min = currentTime - TIME_RANGE - DELAY_SECONDS;
        plot.options.scales.xAxes[0].ticks.max = currentTime - DELAY_SECONDS;
        plot.update();
    }

    function filterXBoundingTicks(tickVal: number, index: number, allTicks: Array<Point>) {
        if (index === 0) {
            return round(tickVal, 1);
        }

        if (index === allTicks.length - 1) {
            return '';
        }

        return tickVal;
    }

    function filterYBoundingTicks(tickVal: number, index: number, allTicks: Array<Point>) {
        if (index === 0) {
            return null;
        }

        if (index === allTicks.length - 1 && tickVal < 0) {
            return '';
        }

        return tickVal;
    }

    function createPlot(plotId: string, name: string, suggestedYRange: Array<number>) {
        return new Chart((document.getElementById(plotId) as HTMLCanvasElement).getContext('2d'), {
            type: 'scatter',
            data: {
                datasets: [
                    {
                        borderColor: 'rgba(66, 134, 244, 1)',
                        backgroundColor: 'rgba(66, 134, 244, 1)',
                        label: name,
                        fill: false,
                        data: []
                    }
                ]
            },
            options: {
                showLines: true,
                responsive: false,
                animation: {
                    duration: 0
                },
                events: 'click',
                elements: {
                    point: {
                        radius: 0,
                        hitRadius: 0
                    },
                    line: {
                        tension: 0
                    }
                },
                scales: {
                    xAxes: [
                        {
                            ticks: {
                                maxRotation: 0,
                                min: -100,
                                max: 0,
                                callback: filterXBoundingTicks
                            },
                        }
                    ],
                    yAxes: [
                        {
                            ticks: {
                                min: suggestedYRange[0],
                                max: suggestedYRange[1],
                                callback: filterYBoundingTicks
                            }
                        }
                    ]
                }
            }
        } as any);
    }

    let currentElapsedTime: number;
    let lastUpdateTime: number;

    function updatePlots(timestamp: number) {
        requestAnimationFrame(updatePlots);

        if (!anyDataReceived) {
            return;
        }

        const currentDataTime = time[time.length - 1];
        if (!currentElapsedTime) {
            currentElapsedTime = currentDataTime;
        } else {
            currentElapsedTime += (timestamp - lastUpdateTime) / 1000;
        }
        lastUpdateTime = timestamp;

        let plotUpdated = false;
        if (timestamp - lastDataUpdateTime > DATA_UPDATE_INTERVAL) {
            updatePlotData(speedPlot, speed);
            updatePlotData(throttlePlot, throttle);
            updatePlotData(compressedPlot, compressed);
            updatePlotData(brakePlot, brake);
            updatePlotData(gearPlot, gear);
            updatePlotData(steeringPlot, steering);

            plotUpdated = true;
            lastDataUpdateTime = timestamp;
        }

        if (timestamp - lastScaleUpdateTime > SCALE_UPDATE_INTERVAL) {
            timeElem.innerHTML = 'Data Time: ' + round(currentDataTime, 3) + ' Elapsed: ' + round(currentElapsedTime, 3) + ' Drift: ' + round(currentDataTime - currentElapsedTime, 3);
            updatePlotScale(speedPlot, currentDataTime);
            updatePlotScale(throttlePlot, currentDataTime);
            updatePlotScale(compressedPlot, currentDataTime);
            updatePlotScale(brakePlot, currentDataTime);
            updatePlotScale(gearPlot, currentDataTime);
            updatePlotScale(steeringPlot, currentDataTime);

            plotUpdated = true;
            lastScaleUpdateTime = timestamp;
        }

        if (plotUpdated) {
            framesSinceUpdate++;
        }
        if (timestamp - lastFPSUpdateTime > FPS_UPDATE_INTERVAL) {
            fpsElem.innerHTML = 'FPS: ' + round((framesSinceUpdate / FPS_UPDATE_INTERVAL) * 1000);
            totalPointsElem.innerHTML = 'Total points: ' + totalDataPoints;
            displayedPointsElem.innerHTML = ' Total in range (uncompressed): ' + time.length;
            speedPointsElem.innerHTML = 'Speed displayed: ' + speed.length;
            throttlePointsElem.innerHTML = 'Throttle displayed: ' + throttle.length;
            brakePointsElem.innerHTML = 'Brake displayed: ' + brake.length;
            gearPointsElem.innerHTML = 'Gear displayed: ' + gear.length;
            steeringPointsElem.innerHTML = 'Steering displayed: ' + steering.length;
            compressedPointsElem.innerHTML = 'Compressed displayed: ' + compressed.length;

            framesSinceUpdate = 0;
            lastFPSUpdateTime = timestamp;
        }
    }

    speedPlot = createPlot('speed-plot', 'Speed(mph)', [0, 250]);
    throttlePlot = createPlot('throttle-plot', 'Throttle', [-0.05, 1.05]);
    compressedPlot = createPlot('compressed-plot', 'Compressed', [0, 250]);
    brakePlot = createPlot('brake-plot', 'Brake', [-0.05, 1.05]);
    gearPlot = createPlot('gear-plot', 'Gear', [-1.2, 9.2]);
    steeringPlot = createPlot('steering-plot', 'Steering', [-1, 1]);

    requestAnimationFrame(updatePlots);
    core.replayAllLaps();
}

interface Point {
    x: number;
    y: number;
}

type Plot = Chart & ChartConfiguration;