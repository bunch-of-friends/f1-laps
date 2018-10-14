import moment from 'moment';

import { idStringMap } from 'f1-laps-js-bridge';

export function formatValue(key: string, value: boolean | string | number) {
    if (typeof value === 'boolean') {
        return value ? 'true' : 'false';
    } else if (typeof value === 'string') {
        return value;
    } else {
        const mapped = idToString(key, value);
        if (typeof mapped === 'string') {
            return mapped;
        } else if (mapped % 1 !== 0) {
            return mapped.toFixed(3);
        } else {
            return mapped;
        }
    }
}

export function idToString(key: string, value: number): number | string {
    if (idStringMap[key]) {
        return idStringMap[key][value] || '';
    } else {
        return value;
    }
}

export function nanosecondsToDate(value: number): Date {
    return new Date(value / 1000000);
}

export function nanosecondsToString(value: number, format: string): string {
    return moment(nanosecondsToDate(value)).format(format);
}

export function secondsToString(value: number): string {
    const minutes = Math.floor(value / 60);
    const seconds = value - minutes * 60;
    const minutesString = minutes > 0 ? `${minutes}:` : '';
    const secondsString =
        seconds >= 10
            ? `${formatDecimal(seconds)}`
            : `0${formatDecimal(seconds)}`;
    return minutesString + secondsString;
}

export function formatDecimal(value: number) {
    return value.toFixed(3);
}
