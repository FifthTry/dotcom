class LoaderComponent extends HTMLElement {
    connectedCallback() {
        console.log("Loader component initialized")
        // Always call super first in constructor
        let data = window.ftd.component_data(this);
        const version = data.version ? data.version.get() : 'v1';
        const method = data.method ? data.method.get() : 'get';
        const headers = data.headers ? data.headers : null;
        console.log(data);
        switch(version) {
            case 'v1':
                // Using http from function.js
                http(data.url.get(), method, null, null)
            break;
            case 'v2':
                http2(data.url.get(), method, null, null, data.return_variable);
            break;
            case 'v3':
                http(data.url.get(), method, headers, null);
            break;
        }

        // // Create a shadow root
        // const shadow = this.attachShadow({mode: 'open'});

    }
}
customElements.define('loader-component', LoaderComponent);


function http2(url, method, headers, body, return_variable) {
    console.log(url)
    if (url instanceof fastn.mutableClass) url = url.get();
    if (method instanceof fastn.mutableClass) method = method.get();
    method = method.trim().toUpperCase();
    const init = {
        method,
        headers: {}
    };
    if(headers && headers instanceof fastn.recordInstanceClass) {
        Object.assign(init.headers, headers.toObject());
    }
    if(method !== 'GET') {
        init.headers['Content-Type'] = 'application/json';
    }
    if(body && body instanceof fastn.recordInstanceClass && method !== 'GET') {
        init.body = JSON.stringify(body.toObject());
    }

    let json;

    fetch(url, init)
        .then(res => {
            if(!res.ok) {
                return new Error("[http]: Request failed", res)
            }

            return res.json();
        })
        .then(output => {
            console.log("[http]: Response OK", output);
            console.log(return_variable)
            if(return_variable instanceof MutableListVariable) {
                output
                    .data
                    .forEach((item, index) => return_variable.insertAt(index, item));
            } else {
                return_variable.set(output);
            }
        })
        .catch(console.error);
    return json;
}
