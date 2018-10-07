import { h } from 'hyperapp';

export const ObjectView = ({
    title,
    data,
}: {
    title: string;
    data?: Object;
}) => (
    <div>
        <span class="debug-title">{title}</span>
        {renderData(data)}
    </div>
);

function renderData(data?: { [key: string]: any }) {
    if (data) {
        return (
            <p>
                {Object.keys(data).map(key => {
                    return (
                        <span>
                            <span>{key}: </span>
                            <span>{renderProp(data[key])}</span>
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

function renderProp(data: any) {
    if (Array.isArray(data)) {
        return data.map(x => {
            return <p class="debug-nested-prop">{renderProp(x)}</p>;
        });
    } else if (typeof data === 'object') {
        return <p class="debug-nested-prop">{renderData(data)}</p>;
    } else {
        return (
            <span>
                {data}
                <br />
            </span>
        );
    }
}
