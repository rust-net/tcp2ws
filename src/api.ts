const base = 'http://127.0.0.1:6601/';

function api([uri]: TemplateStringsArray) {
    return base + uri;
}

type Config = {
    port: number,
    item: {
        ws: string,
        listen: string,
    }[],
}

export default {
    config(): Promise<Config> {
        return fetch(api `api/config`).then(r => r.json())
    },
    exit() {
        return fetch(api `exit`).then(r => r.text())
    }
}