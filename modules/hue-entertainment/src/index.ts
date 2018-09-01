import * as request from 'request';
import { dtls } from 'node-dtls-client';

export const HUE_COLORS = {
    RED: [0xff, 0xff, 0x00, 0x00, 0x00, 0x00],
    BLUE: [0x00, 0x00, 0x00, 0x00, 0xff, 0xff]
};

export type HueColor = Array<number>;

export interface HueConfig {
    bridgeIp: string;
    timeout: number;
}

export class HueEntertainment {
    private clientKey: Buffer;
    private userName: string;
    private currentColor = HUE_COLORS.BLUE;

    constructor(private config: HueConfig) { }

    // Connect to Hue Bridge
    // POST http://<bridgeIp>/api
    public connect(deviceName: string): Promise<void> {
        const options = { json: true, body: { devicetype: 'f1laps#' + deviceName, generateclientkey: true } };

        return new Promise((resolve, reject) => {
            request.post('http://' + this.config.bridgeIp + '/api', options, (err, response, body) => {
                // TODO: handle errors
                if (response && response.statusCode === 200) {
                    this.clientKey = new Buffer(body[0].success.clientKey, 'hex');
                    this.userName = body[0].success.username;
                    resolve();
                } else {
                    reject();
                }
            });
        });
    }

    // Get available groups
    // GET http://<bridgeIp>/api/<username>/groups
    public getGroups(): Promise<Array<string>> {
        return new Promise((resolve) => {
            request.get('http://' + this.config.bridgeIp + '/api/' + this.userName + '/groups', (err, response, body) => {
                const entertainmentGroups: Array<string> = [];
                Object.keys(body).forEach((key) => {
                    if (body[key].type === 'entertainment') {
                        entertainmentGroups.push(body[key]);
                    }
                });
                resolve(entertainmentGroups);
            });
        });
    }

    // Start streaming session
    // DTLS UDP Port 2100
    public startSession(group: string): Promise<void> {
        return new Promise((resolve, reject) => {
            const options = { json: true, body: { stream: { active: true } } };
            request.put('http://' + this.config.bridgeIp + '/api/' + this.userName + '/groups/' + group, options, (err, response, body) => {
                // TODO: handle errors
                if (response && response.statusCode === 200) {
                    return this.startStreaming(group);
                }
            });
        });
    }

    // Change current color
    public changeColor(color: HueColor, opacity = 1) {
        // TODO: opacity
        this.currentColor = color;
    }

    private startStreaming(group: string): Promise<void> {
        const psk: { [key: string]: string } = {};
        psk[this.userName] = this.clientKey.toString(); // conversion to string necessary?
        const options: dtls.Options = { type: 'udp4', address: this.config.bridgeIp, port: 2100, psk: psk, timeout: this.config.timeout };
        return new Promise((resolve, reject) => {
            const socket = dtls.createSocket(options)
            .on('connected', () => {
                setInterval(() => socket.send(this.constructBuffer(group)), 5000);
                resolve();
            })
            .on('error', (error) => {
                reject(error);
            })
            .on('message', (msg) => {
                console.log('[socket] message: ', msg); /* tslint:disable-line:no-console */
            })
            .on('close', () => {
                console.log('[socket] closed'); /* tslint:disable-line:no-console */
            });
        });
    }

    private constructBuffer(group: string) {
        return Buffer.concat([
            Buffer.from('HueStream', 'ascii'), // Protocol name
            Buffer.from([
                0x01, 0x00, // Streaming API version
                0x07, // Sequence ID
                0x00, 0x00, // Reserved
                0x00, // Color space
                0x00, // Reserved
                0x00, // Type of device (0x00 = Light)
                0x00, 0x03 // Unique ID of light TODO: convert light ID to bytes
            ]),
            Buffer.from(this.currentColor) // RGB or XY+Brightness with 16 bit resolution per element
        ]);
    }
}
