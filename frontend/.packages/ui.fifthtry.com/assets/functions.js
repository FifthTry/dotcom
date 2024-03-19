function get_initials(name) {
    return fastn_utils.getStaticValue(name)
        .split(" ")
        .map((p) => p[0].toUpperCase())
        .join("");
}

function callAlert(msg) {
    alert(msg)
}

function createCookie(name, value, days) {
    if (days) {
        var date = new Date();
        date.setTime(date.getTime() + (days * 24 * 60 * 60 * 1000));
        var expires = "; expires=" + date.toGMTString();
    } else {
        var expires = "";
    }
    document.cookie = name + "=" + value + expires + "; path=/";
}

function eraseCookie(name) {
    createCookie(name, "", -1);
}

function set_language(language) {
    eraseCookie("fastn-lang");
    createCookie("fastn-lang", language, 365 * 60);
    document.location.reload();
}
