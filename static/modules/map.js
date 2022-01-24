const uri = 'ws://' + location.host + '/connect';
const svgns = "http://www.w3.org/2000/svg";
const MAX_ZOOM = 50.0;

function connect(svg) {

    const ws = new WebSocket(uri);
    const renderer = new MapRenderer(svg);

    ws.onopen = function () {
        console.log("Connected to update server");
    }

    ws.onmessage = function (msg) {
        let players = JSON.parse(msg.data);
        for (const player of players) {
            renderer.update(player);
        }
    }

    return renderer;
}

function MapRenderer(svg) {

    var self = this;

    self.svg = svg;
    self.players = {};
    self.loaded = false;

    document.querySelector(self.svg).addEventListener('load', function () {
        self.mapRef = svgPanZoom(svg, {
            zoomEnabled: true,
            maxZoom: MAX_ZOOM,
            zoomScaleSensitivity: 0.3,
        });
        self.svgDom = document.querySelector(self.svg).getSVGDocument();
        self.loaded = true;
    });

    self.update = function (player) {
        if (!self.loaded) {
            return;
        }
        if (!self.players.hasOwnProperty(player.name)) {
            self.create_marker(player);
        }
        self.update_marker(player);
    }

    self.create_marker = function (player) {
        const {
            name,
            character_name,
            position
        } = player;
        var marker = document.createElementNS(svgns, 'circle');
        marker.setAttribute('cx', position.x);
        marker.setAttribute('cy', position.y);
        marker.setAttribute('r', 10);
        marker.setAttribute('fill', '#8e44ad');

        self.players[name] = {
            name,
            character_name,
            position,
            marker,
        }
        self.svgDom.documentElement.children[0].appendChild(marker);
        // console.log("Created a new marker for player ", name);
    }

    self.update_marker = function (player) {
        const {
            name,
            position
        } = player;
        const marker = self.players[name].marker;
        self.players[name].position = position;
        marker.setAttribute('cx', position.x);
        marker.setAttribute('cy', position.y);
        //  console.log("Updated marker for player ", name);
    }

    self.get_player = function (name) {
        if (!self.players.hasOwnProperty(name)) {
            console.error("No player found with the name ", name);
            return null;
        }
        return self.players[name];
    }

    self.focus_player = function (name) {
        const player = self.get_player(name);
        if (player != null) {
            showNode(player.marker.getBBox(), self.mapRef);
        }
    }

    self.center_on = function (point) {
        const { width, height, realZoom } = self.mapRef.getSizes()
        self.mapRef.pan({
            x: -realZoom * (point.x - width / (realZoom * 2)),
            y: -realZoom * (point.y - height / (realZoom * 2))
        })
    }
}

function showNode(bbox, svgPan, zoom = true) {
    // From https://github.com/bumbu/svg-pan-zoom/issues/381

    const { width, height, realZoom } = svgPan.getSizes()
    svgPan.pan({
        x: -realZoom * (bbox.x - width / (realZoom * 2) + bbox.width / 2),
        y: -realZoom * (bbox.y - height / (realZoom * 2) + bbox.height / 2)
    })

    if (zoom) {
        const relativeZoom = svgPan.getZoom();
        const desiredWidth = MAX_ZOOM * Math.sqrt(bbox.width / 25) * 11 * realZoom;
        svgPan.zoom(relativeZoom * width / desiredWidth)
    }
}

export { connect };