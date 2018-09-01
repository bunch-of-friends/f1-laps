// POST http://192.168.0.53/api
// body:
// {
//   "devicetype":"f1laps#jamiemaison",
//   "generateclientkey":true
// } 

let request = require ('request');
let dtls = require ('node-dtls-client').dtls;

const hue_hub = '192.168.0.53';
const hue_clientkey = new Buffer ('4C9C5A49A4E0E2A1CEDF369B16AFA6FC', 'hex');
const hue_username = '2yV81hK-AKsVDbMEceNmACqUCHzLdXivCVITRrUD';

const RED = [0xff, 0xff, 0x00, 0x00, 0x00, 0x00]
const BLUE = [0x00, 0x00, 0x00, 0x00, 0xff, 0xff]
let color = RED

setInterval(() => {
    if (color === RED) {
        color = BLUE;
    } else {
        color = RED;
    }
}, 30)

let options = {
  method: 'PUT',
  url: 'http://' + hue_hub + '/api/' + hue_username + '/groups/3',
  json: true,
  body: {
    stream: {
      active: true,
    },
  },
};
request (options, (err, response, body) => {
  console.log (response.statusCode);
  if (response && response.statusCode < 400) {
    let options = {
      type: 'udp4',
      address: hue_hub,
      port: 2100,
      psk: {},
      timeout: 1000,
    };
    options.psk[hue_username] = hue_clientkey;
    let socket = dtls
      .createSocket (options)
      .on ('connected', () => {
        setInterval(() => {
            const msg = Buffer.concat ([
                Buffer.from ('HueStream', 'ascii'),
                Buffer.from([
                    0x01, 0x00,
                    0x07,
                    0x00, 0x00,
                    0x00,
                    0x00,
                    0x00, 0x00, 0x03
                ]),
                Buffer.from(color)
              ]);
              socket.send(msg);
        }, 20);
      })
      .on ('error', (error) => {
        console.log ('[socket] error: ', error);
      })
      .on ('message', (msg) => {
        console.log ('[socket] message: ', msg);
      })
      .on ('close', () => {
        console.log ('[socket] closed');
      });
  }
});
