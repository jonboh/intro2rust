import 'reveal.js/dist/reset.css'
import 'reveal.js/dist/reveal.css'
// see available themes in the
// node_modules/reveal.js/dist/theme
//  beige, black, blood, league, moon, night, serif, simple, ...
import 'reveal.js/dist/theme/black.css'
// our styles to adjust the presentation
import './style/tokio-night-dark.css'
import Reveal from 'reveal.js'
import RevealHighlight from "reveal.js/plugin/highlight/highlight";

import ExternalCode from '@edc4it/reveal.js-external-code';

const deck = new Reveal()
deck.initialize({
        width: "50%",
        height: "50%",

        hash: true,
        // history: true,
        // overview: false,
        center: true,
        // margin: 0.15,
        navigationMode: "linear",

        externalCode: {
            basePath: "/intro2rust"
        },
        plugins: [ExternalCode, RevealHighlight],
      });
