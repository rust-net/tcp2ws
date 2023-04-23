const base = './';

function api([uri]: TemplateStringsArray) {
    return base + uri;
}

export type Item = {
    name: string,
    desc: string,
    ws: string,
    listen: string,
}

export type Config = {
    port: number,
    item: Item[],
}

function post(url: string, json: object) {
    return fetch(url, {
        method: 'POST',
        body: JSON.stringify(json),
        headers: {
            'Content-Type': 'application/json',
        },
    }).then(async r => r.ok ? r.text() : Promise.reject(await r.text()));
}

export default {
    get_config(): Promise<Config> {
        return fetch(api `api/config`).then(r => r.json())
    },
    set_config(config: Config) {
        return post(api `api/config`, config);
    },
    start(item: Item) {
        return post(api `api/start`, item);
    },
    stop(item: Item) {
        return post(api `api/stop`, item);
    },
    list(): Promise<Item[]> {
        return fetch(api `api/list`).then(r => r.json())
    },
    exit() {
        return fetch(api `exit`).then(r => r.text())
    },
    compareItems(item1: Item, item2: Item): boolean {
        return (
            item1.name === item2.name &&
            item1.desc === item2.desc &&
            item1.ws === item2.ws &&
            item1.listen === item2.listen
        )
    },
}