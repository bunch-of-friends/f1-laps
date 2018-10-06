import { h } from 'hyperapp';

export const ObjectView = ({
    title,
    data,
}: {
    title: string;
    data?: Object;
}) => (
    <div>
        <h4>{title}</h4>
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
                            <br />
                        </span>
                    );
                })}
            </p>
        );
    } else {
        return <p>---</p>;
    }
}

function renderProp(data: any) {
    if (Array.isArray(data)) {
        return data.map(x => {
            return <div class="debug-nested-prop">{renderProp(x)}</div>;
        });
    } else if (typeof data === 'object') {
        return <div class="debug-nested-prop">{renderData(data)}</div>;
    } else {
        return <span>{data}</span>;
    }
}
