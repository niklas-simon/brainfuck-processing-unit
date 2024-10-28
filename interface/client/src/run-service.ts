const _fetch = async (url: string, init?: RequestInit) => {
    //await new Promise(res => setTimeout(() => res(true), Math.random() * 2000));
    return fetch(url, init);
}

export interface Example {
    name: string,
    desc: string,
    code: string
}

export type State = {
    tape?: number[],
    head?: number,
    code?: {
        pc: number,
        offset: number,
        fragment: string
    },
    ic?: number,
    jumping?: null | number,
    stack?: number[],
    cycles?: number,
    control?: "idle" | "uncontrolled" | "paused" | "running" | "startup" | "wait_input" | "output_ready"
}

export enum Action {
    PLAY,
    PAUSE,
    STEP,
    RESET
}

const actionMap = new Map([
    [Action.PLAY, "start"],
    [Action.PAUSE, "pause"],
    [Action.STEP, "step"],
    [Action.RESET, "reset"]
]);

const onError = (e: Event) => console.error(e);

export default class RestService {
    private static instance?: RestService;

    private examples = _fetch("/api/examples").then(res => res.json());

    private programEvent = new EventSource("/api/sse/code");

    private inputEvent = new EventSource("/api/sse/input");

    private outputEvent = new EventSource("/api/sse/output");

    private stateEvent = new EventSource("/api/sse/state");

    private speedEvent = new EventSource("/api/sse/speed");

    private constructor() {
        [this.programEvent, this.inputEvent, this.outputEvent, this.stateEvent, this.speedEvent].forEach(e => e.addEventListener("error", onError));
    }

    public static getInstance() {
        if (!this.instance) {
            this.instance = new RestService();
        }

        return this.instance;
    }

    public getExamples() {
        return this.examples;
    }

    public async getProgram() {
        const res = await _fetch("/api/run/code");
        return await res.text();
    }

    public setProgram(code: string) {
        return _fetch("/api/run/code", {
            method: "PUT",
            body: code
        });
    }

    public getProgramEvent() {
        return this.programEvent;
    }

    public async getInput() {
        const res = await _fetch("/api/run/input");
        return await res.text();
    }

    public setInput(input: string) {
        return _fetch("/api/run/input", {
            method: "PUT",
            body: input
        });
    }

    public getInputEvent() {
        return this.inputEvent;
    }

    public async getOutput() {
        const res = await _fetch("/api/run/output");
        return await res.text();
    }

    public getOutputEvent() {
        return this.outputEvent;
    }

    public async getState() {
        const res = await _fetch("/api/run/state");
        return await res.json() as State;
    }

    public getStateEvent() {
        return this.stateEvent;
    }

    public async getSpeed() {
        const res = await _fetch("/api/run/speed");
        return parseInt(await res.text());
    }

    public setSpeed(speed: number) {
        return _fetch("/api/run/speed", {
            method: "PUT",
            body: speed.toString()
        });
    }

    public getSpeedEvent() {
        return this.speedEvent;
    }

    public setControl(control: boolean) {
        return _fetch("/api/ctrl", {
            method: control ? "PUT" : "DELETE"
        });
    }

    public controlAction(action: Action) {
        return _fetch("/api/ctrl/" + actionMap.get(action)!, {
            method: "POST"
        });
    }
}