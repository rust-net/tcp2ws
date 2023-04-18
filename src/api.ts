const base = 'http://127.0.0.1:6601/';

function api([uri]: TemplateStringsArray) {
    return base + uri;
}

export default {
    exit() {
        fetch(api `exit`).then(r => r.text()).then(r => {
            console.log(r);
        })
    }
}