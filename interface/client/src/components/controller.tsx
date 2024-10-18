import { Button, ButtonGroup, Slider, Switch } from "@nextui-org/react";
import { useState } from "react";
import { Pause, Play, RefreshCcw, SkipForward } from "react-feather";

enum State {
    PAUSED,
    RUNNING
}

export default function Controller() {
    const [control, setControl] = useState(false);
    const [state, setState] = useState(State.PAUSED);

    return <div className="flex flex-col gap-4">
        <Switch isSelected={control} onValueChange={setControl}>Enable machine control</Switch>
        <ButtonGroup className="w-min">
            <Button isIconOnly color="success" variant="ghost"
                isDisabled={state === State.RUNNING || !control}
                onClick={() => setState(State.RUNNING)}>
                <Play/>
            </Button>
            <Button isIconOnly color="warning" variant="ghost"
                isDisabled={state === State.PAUSED || !control}
                onClick={() => setState(State.PAUSED)}>
                <Pause/>
            </Button>
            <Button isIconOnly color="primary" variant="ghost"
                isDisabled={state !== State.PAUSED || !control}>
                <SkipForward/>
            </Button>
            <Button isIconOnly color="danger" variant="ghost"
                isDisabled={!control}>
                <RefreshCcw/>
            </Button>
        </ButtonGroup>
        <Slider label="Speed" step={1} maxValue={100} minValue={1} defaultValue={100} className="max-w-[500px]" isDisabled={!control}/>
    </div>
}