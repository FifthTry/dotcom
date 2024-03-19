(function() {
    if (window.svg_helper_loaded) {
        // TODO: why do we need this?
        return;
    }
    window.svg_helper_loaded = true;
    console.log("svg helper loaded");

    let svg_cache = {};
    let svg_waiting = {};
    let logging_enabled = false;

    function log(msg) {
        if (logging_enabled) {
            console.log(msg);
        }
    }


    // if you update this also update ftd file also
    let SVG_ICON_CLASS = "fastn-community-svg-icon-helper";

    function update_svg(el) {
        let url = el.innerText;

        if (!url) {
            return;
        }

        if (url in svg_cache) {
            log("svg_cache hit for " + url);
            el.innerHTML = svg_cache[url];
            return;
        }

        log("svg_cache miss for " + url);
        if (url in svg_waiting) {
            log("svg_waiting hit for " + url);
            svg_waiting[url].push(el);
            return;
        }

        svg_waiting[url] = [el];
        fetch("https://" + url).then(function (response) {
            console.log("response", response);
            if (response.status !== 200) {
                console.log("response.status", response.status);
                return;
            }
            return response.text();
        }).then(function (text) {
            if (!text) {
                // this happens when we get 404
                return;
            }
            if (text.indexOf("currentColor") === -1) {
                text = text.replace(/<svg /g, '<svg fill="currentColor" ');
            }
            svg_cache[url] = text;
            for (let el of svg_waiting[url]) {
                el.outerHTML = text;
            }
            delete svg_waiting[url];
        });

    }


    // get class that starts with fa-
    function update_all_svg() {
        log("update_all_svg");
        for (let el of document.getElementsByClassName(SVG_ICON_CLASS)) {
            update_svg(el);
        }
        let observer = new MutationObserver(function (mutations) {
            for (let mutation of mutations) {
                let el = mutation.target;
                if (el.nodeName === "div" && el.classList.contains(SVG_ICON_CLASS)) {
                    update_svg(el);
                }
            }
        });
        let config = {attributes: true, childList: true, subtree: true};
        observer.observe(document.body, config);
    }


    window.addEventListener("load", update_all_svg);
})();
