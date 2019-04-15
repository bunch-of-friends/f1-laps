import { h } from 'hyperapp';
import { formatValue } from '../helpers/formatting';

export const ObjectView = ({
    title,
    data,
}: {
    title: string;
    data?: Object;
}) => (
    <div>
        <span class="debug-title">{title}</span>
        {renderObject(data)}
    </div>
);

function renderObject(obj?: { [key: string]: any }) {
    if (obj) {
        return (
            <p>
                {Object.keys(obj).map(key => {
                    return (
                        <span>
                            <span>{key}: </span>
                            <span>{renderProp(key, obj[key])}</span>
                        </span>
                    );
                })}
                <br />
            </p>
        );
    } else {
        return <p>---</p>;
    }
}

function renderProp(key: string, value: any) {
    if (Array.isArray(value)) {
        return (
            <p class="debug-nested-prop">
                [
                {value.map(item => {
                    if (typeof item === 'object') {
                        return (
                            <span class="debug-nested-prop">
                                {renderObject(item)},
                            </span>
                        );
                    } else {
                        return (
                            <span class="debug-nested-prop">
                                {formatValue('', item)},
                            </span>
                        );
                    }
                })}
                ]
            </p>
        );
    } else if (typeof value === 'object') {
        return <p class="debug-nested-prop">{renderObject(value)}</p>;
    } else {
        return (
            <span>
                {formatValue(key, value)}
                <br />
            </span>
        );
    }
}
