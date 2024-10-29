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

    private examples = _fetch("/api/examples").then(res => res.json() as Promise<Example[]>);

    private program: string | null = null;

    private input: string | null = null;

    private output: string | null = null;

    private state: State | null = null;

    private speed: number | null = null;

    private programEvent = new EventSource("/api/sse/code");

    private inputEvent = new EventSource("/api/sse/input");

    private outputEvent = new EventSource("/api/sse/output");

    private stateEvent = new EventSource("/api/sse/state");

    private speedEvent = new EventSource("/api/sse/speed");

    private constructor() {
        [this.programEvent, this.inputEvent, this.outputEvent, this.stateEvent, this.speedEvent].forEach(e => e.addEventListener("error", onError));
        window.onbeforeunload = () => [this.programEvent, this.inputEvent, this.outputEvent, this.stateEvent, this.speedEvent].forEach(e => e.close());

        this.programEvent.addEventListener("message", e => {
            this.program = e.data;
            window.dispatchEvent(new CustomEvent("program", {
                detail: this.program
            }));
        });
        this.inputEvent.addEventListener("message", e => {
            this.input = e.data;
            window.dispatchEvent(new CustomEvent("input", {
                detail: this.input
            }));
        });
        this.outputEvent.addEventListener("message", e => {
            this.output = e.data;
            window.dispatchEvent(new CustomEvent("output", {
                detail: this.output
            }));
        });
        this.stateEvent.addEventListener("message", e => {
            this.state = JSON.parse(e.data) as State;
            window.dispatchEvent(new CustomEvent("state", {
                detail: this.state
            }));
        });
        this.speedEvent.addEventListener("message", e => {
            this.speed = parseInt(e.data);
            window.dispatchEvent(new CustomEvent("speed", {
                detail: this.speed
            }));
        });

        _fetch("/api/run/code").then(res => res.text()).then(res => {
            if (!this.program) {
                this.program = res;
                window.dispatchEvent(new CustomEvent("program", {
                    detail: this.program
                }));
            }
        });
        _fetch("/api/run/input").then(res => res.text()).then(res => {
            if (!this.input) {
                this.input = res;
                window.dispatchEvent(new CustomEvent("input", {
                    detail: this.input
                }));
            }
        });
        _fetch("/api/run/output").then(res => res.text()).then(res => {
            if (!this.output) {
                this.output = res;
                window.dispatchEvent(new CustomEvent("output", {
                    detail: this.output
                }));
            }
        });
        _fetch("/api/run/state").then(res => res.json() as Promise<State>).then(res => {
            if (!this.state) {
                this.state = res;
                window.dispatchEvent(new CustomEvent("state", {
                    detail: this.state
                }));
            }
        });
        _fetch("/api/run/speed").then(res => res.text()).then(res => parseInt(res)).then(res => {
            if (!this.speed) {
                this.speed = res;
                window.dispatchEvent(new CustomEvent("speed", {
                    detail: this.speed
                }));
            }
        });
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

    public getProgram() {
        return this.program;
    }

    public setProgram(code: string) {
        return _fetch("/api/run/code", {
            method: "PUT",
            body: code
        });
    }

    public onProgramChange(callback: (program: string) => void) {
        const profile = {
            name: "program",
            callback: (e: Event) => callback((e as CustomEvent).detail)
        }
        window.addEventListener(profile.name, profile.callback);
        return profile;
    }

    public getInput() {
        return this.input;
    }

    public setInput(input: string) {
        return _fetch("/api/run/input", {
            method: "PUT",
            body: input
        });
    }

    public onInputChange(callback: (input: string) => void) {
        const profile = {
            name: "input",
            callback: (e: Event) => callback((e as CustomEvent).detail)
        }
        window.addEventListener(profile.name, profile.callback);
        return profile;
    }

    public getOutput() {
        return this.output;
    }

    public onOutputChange(callback: (output: string) => void) {
        const profile = {
            name: "output",
            callback: (e: Event) => callback((e as CustomEvent).detail)
        }
        window.addEventListener(profile.name, profile.callback);
        return profile;
    }

    public getState() {
        return this.state;
    }

    public onStateChange(callback: (state: State) => void) {
        const profile = {
            name: "state",
            callback: (e: Event) => callback((e as CustomEvent).detail as State)
        }
        window.addEventListener(profile.name, profile.callback);
        return profile;
    }

    public getSpeed() {
        return this.speed;
    }

    public setSpeed(speed: number) {
        return _fetch("/api/run/speed", {
            method: "PUT",
            body: speed.toString()
        });
    }

    public onSpeedChange(callback: (speed: number) => void) {
        const profile = {
            name: "speed",
            callback: (e: Event) => callback((e as CustomEvent).detail as number)
        }
        window.addEventListener(profile.name, profile.callback);
        return profile;
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

    public removeListener(profile: {name: string, callback: (e: Event) => void}) {
        window.removeEventListener(profile.name, profile.callback);
    }
}