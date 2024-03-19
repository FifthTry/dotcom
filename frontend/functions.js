function http(url, method, headers, ...body) {
    if (url instanceof fastn.mutableClass) url = url.get();
    if (method instanceof fastn.mutableClass) method = method.get();
    method = method.trim().toUpperCase();
    const init = {
        method,
        headers: {}
    };
    if (headers && headers instanceof fastn.recordInstanceClass) {
        Object.assign(init.headers, headers.toObject());
    }
    if (method !== 'GET') {
        init.headers['Content-Type'] = 'application/json';
    }
    if (body && body instanceof fastn.recordInstanceClass && method !== 'GET') {
        init.body = JSON.stringify(body.toObject());
    } else if (body && method !== 'GET') {
        let json = body[0];
        if (body.length !== 1 || (body[0].length === 2 && Array.isArray(body[0]))) {
            let new_json = {};
            // @ts-ignore
            for (let [header, value] of Object.entries(body)) {
                let [key, val] = value.length == 2 ? value : [header, value];
                new_json[key] = fastn_utils.getStaticValue(val);
            }
            json = new_json;
        }
        init.body = JSON.stringify(json);
    }

    let json;

    fetch(url, init)
        .then(res => {
            if (!res.ok) {
                return new Error("[http]: Request failed", res)
            }

            return res.json();
        })
        .then(response => {
            console.log("[http]: Response OK", response);
            if (response.redirect) {
                window.location.href = response.redirect;
            }
            else if (!!response && !!response.reload) {
                window.location.reload();
            } else {
                let data = {};
                if (!!response.errors) {
                    for (let key of Object.keys(response.errors)) {
                        let value = response.errors[key];
                        if (Array.isArray(value)) {
                            // django returns a list of strings
                            value = value.join(" ");
                            // also django does not append `-error`
                            key = key + "-error";
                        }
                        // @ts-ignore
                        data[key] = value;
                    }
                }
                if (!!response.data) {
                    if (Object.keys(data).length !== 0) {
                        console.log("both .errrors and .data are present in response, ignoring .data");
                    }
                    else {
                        data = response.data;
                    }
                }
                console.log(response)
                for (let ftd_variable of Object.keys(data)) {
                    // @ts-ignore
                    window.ftd.set_value(ftd_variable, data[ftd_variable]);
                }
            }
        })
        .catch(console.error);
    return json;
}

/**
 * @param {MutableVariable} name
 *
 * John Doe -> JD
 */
function get_initials(name) {
    return fastn_utils.getStaticValue(name)
        .split(" ")
        .map((p) => p[0])
        .join("");
}

function set_current_template(current_template, template) {
    for (const [key, value] of Object.entries(template.getAllFields())) {
        current_template.set(key, key === 'thumbnails' ? value.get() : value)
    }
}


function set_current_thumbnail(current_thumbnail, thumbnail) {
    console.log(current_thumbnail, thumbnail)
    current_thumbnail.set(thumbnail)
}


async function create_presentation_from_template(template) {
    const templateId = fastn_utils.getFlattenStaticValue(template.get('id'));

    const response = await fetch(`create-presentation/${templateId}`, {
        method: "POST",
    });

    if (!response.ok) {
        console.error("[HTTP_ERROR]: Could not create presentation");
        return;
    }

    const { data: { presentation_id } } = await response.json();

    location.href = `/p/${presentation_id}/1`;
}

function goBack() {
  window.history.back();
}

// clear error messages before sending the request
function send_form_request(...args) {
    const [error_vars, ...rest] = args;

    for (const variable of error_vars) {
        variable.set(null);
    }

    ftd.http(...rest);
}
