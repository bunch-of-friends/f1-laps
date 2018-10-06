import { h } from 'hyperapp';
import { Point, isColinear } from '../helpers/linear-algebra';
import { round } from 'lodash';
import Chart, { ChartConfiguration } from 'chart.js';
import { AppState, ActivePlot } from '../app/app-state';
import { AppActions } from '../app/app-actions';
import { LiveTelemetryTick } from 'f1-laps-js-bridge';

const CHART_RANGE = 60;

function filterXBoundingTicks(tickVal: number, index: number, allTicks: Array<Point>) {
    if (index === 0) {
        return tickVal > 0 ? round(tickVal, 1) : '';
    }

    if (index === allTicks.length - 1 || tickVal < 0) {
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

const createChart = (
    {
        suggestedYRange,
        key,
        label
    }: TelemetryPlotAttributes,
    actions: AppActions
) => (
    canvas: HTMLCanvasElement
) => {
        const context = canvas.getContext('2d');
        if (!context) {
            throw new Error('canvas not found');
        }

        const newChart = new Chart(
            context,
            {
                type: 'scatter',
                data: {
                    datasets: [
                        {
                            borderColor: 'rgba(66, 134, 244, 1)',
                            backgroundColor: 'rgba(66, 134, 244, 1)',
                            label,
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
                    events: ['click'],
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
                                    min: -CHART_RANGE,
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
            } as ChartConfiguration);

        actions.activePlots.plotActive({
            key,
            activePlot: newChart
        });
    }

function toCompressedPoints<T>(
    data: Array<T>,
    pointSelector: (lapTick: T) => Point
) {
    const compressedData = data.slice(0, 2).map(pointSelector);
    for (let i = 2; i < data.length; i++) {
        const c = pointSelector(data[i]);
        const b = compressedData[compressedData.length - 1];
        const a = compressedData[compressedData.length - 2];

        if (isColinear(a, b, c)) {
            compressedData[compressedData.length - 1] = c;
        } else {
            compressedData.push(c);
        }
    }

    return compressedData;
}

const onTelemetryPlotUpdate = (
    currentAttributes: TelemetryPlotAttributes,
    activePlots: { [key: string]: ActivePlot },
    actions: AppActions
) => (
    element: HTMLCanvasElement, oldAttributes: TelemetryPlotAttributes
) => {
        if (oldAttributes.data === currentAttributes.data) {
            return;
        }

        const activePlot = activePlots[currentAttributes.key].instance;
        if (!activePlot.data.datasets || !activePlot.data.datasets.length) {
            return;
        }

        const compressedPoints = toCompressedPoints(
            currentAttributes.data,
            currentAttributes.pointSelector
        )

        activePlot.data.datasets[0].data = compressedPoints;
        if (compressedPoints.length > 0) {
            const currentTime = compressedPoints[compressedPoints.length - 1].x;
            const plotOptions = (activePlot as ChartConfiguration).options;
            if (plotOptions && plotOptions.scales && plotOptions.scales.xAxes) {
                let x = plotOptions.scales.xAxes[0];
                if (x.ticks) {
                    x.ticks.max = currentTime;
                    x.ticks.min = currentTime - CHART_RANGE;
                }
            }
        }
        activePlot.update();

        actions.activePlots.displayedPointsChanged({
            key: currentAttributes.key,
            displayedPoints: compressedPoints.length
        });
    }

export const TelemetryPlot = (
    attributes: TelemetryPlotAttributes
) => (
    { activePlots }: AppState,
    actions: AppActions
) => {
        const activePlot = activePlots[attributes.key];
        return (
            <div>
                {
                    (attributes.debug && activePlot) ? (
                        <table>
                            <th>
                                <td colSpan="2">{attributes.label} debug info</td>
                            </th>
                            <tr>
                                <td>Total Points:</td>
                                <td>{attributes.data.length}</td>
                            </tr>
                            <tr>
                                <td>Displayed points:</td>
                                <td>{activePlots[attributes.key].displayedPoints}</td>
                            </tr>
                        </table>
                    ) : null
                }
                <canvas
                    width="1200"
                    height="200"
                    oncreate={createChart(attributes, actions)}
                    onupdate={onTelemetryPlotUpdate(attributes, activePlots, actions)}
                />
            </div>
        );
    };

export interface TelemetryPlotAttributes {
    suggestedYRange: [number, number];
    data: Array<LiveTelemetryTick>;
    key: string;
    label: string;
    pointSelector: (lapTick: LiveTelemetryTick) => Point;
    debug: boolean;
}
