import { h } from 'hyperapp';
import { Point } from '../math/linear-algebra';
import { round } from 'lodash';
import Chart, { ChartConfiguration } from 'chart.js';
import { AppState } from '../app-state';
import { AppActions } from '../app-actions';

const MAX_CHART_X = 120;

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

const createChart = (
    {
        suggestedYRange,
        key
    }: TelemetryPlotAttributes,
    actions: AppActions
) => (
    canvas: HTMLCanvasElement
) => {
        const newChart = new Chart(
            canvas.getContext('2d'),
            {
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
                                    min: 0,
                                    max: MAX_CHART_X,
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

const onTelemetryPlotUpdate = (
    currentAttributes: TelemetryPlotAttributes,
    activePlots: { [key: string]: Chart }
) => (
    element: HTMLCanvasElement, oldAttributes: TelemetryPlotAttributes
) => {
        if (oldAttributes.data !== currentAttributes.data) {
            const activePlot = activePlots[currentAttributes.key];
            activePlot.data.datasets[0].data = currentAttributes.data;
            activePlot.update();
        }
    }

export const TelemetryPlot = (
    attributes: TelemetryPlotAttributes
) => (
    { activePlots }: AppState,
    actions: AppActions
) => (
    <canvas
        width="1200"
        height="200"
        oncreate={createChart(attributes, actions)}
        onupdate={onTelemetryPlotUpdate(attributes, activePlots)}
    />
);

export interface TelemetryPlotAttributes {
    suggestedYRange: [number, number];
    data: Array<Point>;
    key: string;
}
