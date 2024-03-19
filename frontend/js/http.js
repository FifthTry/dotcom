function http(url, module, method_, ...body) {
    let opts = fastn.recordInstance({
        "method": method_,
        "redirect": "manual",
        "fastn_module": module
    });
    if ((!opts) instanceof fastn.recordInstanceClass) {
        console.info(`opts must be a record instance of
                -- record ftd.http-options:
                string method: GET
                string redirect: manual
                string fastn-module:
            `);
        throw new Error("invalid opts");
    }

    let method = opts.get("method").get();
    let fastn_module = opts.get("fastn_module").get();
    let redirect = opts.get("redirect").get();

    if (!["manual", "follow", "error"].includes(redirect)) {
        throw new Error(
            `redirect must be one of "manual", "follow", "error"`,
        );
    }

    if (url instanceof fastn.mutableClass) url = url.get();
    method = method.trim().toUpperCase();
    let request_json = {};

    const init = {
        method,
        mode: "cors",
        headers: {
            "Content-Type": "application/json",
            "Access-Control-Allow-Origin": "*"
        },
        json: null,
        redirect,
        credentials: 'include' // Add this line to include cookies in the request
    };

    if (body && method !== "GET") {
        if (body[0] instanceof fastn.recordInstanceClass) {
            if (body.length !== 1) {
                console.warn(
                    "body is a record instance, but has more than 1 element, ignoring",
                );
            }
            request_json = body[0].toObject();
        } else {
            let json = body[0];
            if (
                body.length !== 1 ||
                (body[0].length === 2 && Array.isArray(body[0]))
            ) {
                let new_json = {};
                // @ts-ignore
                for (let [header, value] of Object.entries(body)) {
                    let [key, val] =
                        value.length === 2 ? value : [header, value];
                    new_json[key] = fastn_utils.getStaticValue(val);
                }
                json = new_json;
            }
            request_json = json;
        }
    }

    init.body = JSON.stringify(request_json);

    let json;
    fetch(url, init)
        .then((res) => {
            if (res.redirected) {
                window.location.href = res.url;
                return;
            }

            if (!res.ok) {
                return new Error("[http]: Request failed", res);
            }

            return res.json();
        })
        .then((response) => {
            console.log("[http]: Response OK", response);
            if (response.redirect) {
                window.location.href = response.redirect;
            } else if (!!response && !!response.reload) {
                window.location.reload();
            } else {
                let data = {};
                if (!!response.errors) {
                    for (let key of Object.keys(response.errors)) {
                        let value = response.errors[key];
                        if (Array.isArray(value)) {
                            // django returns a list of strings
                            value = value.join(" ");
                        }
                        // also django does not append `-error`
                        key = key + "-error";
                        key = fastn_module + "#" + key;
                        data[key] = value;
                    }
                }
                if (!!response.data) {
                    if (Object.keys(data).length !== 0) {
                        console.log(
                            "both .errors and .data are present in response, ignoring .data",
                        );
                    } else {
                        for (let key of Object.keys(response.data)) {
                            let value = response.data[key];
                            if (Array.isArray(value)) {
                                // django returns a list of strings
                                value = value.join(" ");
                            }
                            // also django does not append `-error`
                            key = fastn_module + "#" + key;
                            data[key] = value;
                        }
                    }
                }
                for (let ftd_variable of Object.keys(data)) {
                    // @ts-ignore
                    window.ftd.set_value(ftd_variable, data[ftd_variable]);
                }
            }
        })
        .catch(console.error);
    return json;
};
