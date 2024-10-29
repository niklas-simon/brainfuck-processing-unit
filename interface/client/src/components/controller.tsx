import RestService, { Action, State } from "@/run-service";
import { ButtonGroup, Button } from "@nextui-org/button";
import { Progress } from "@nextui-org/progress";
import { Skeleton } from "@nextui-org/skeleton";
import { Slider } from "@nextui-org/slider";
import { Spinner } from "@nextui-org/spinner";
import { Switch } from "@nextui-org/switch";
import { useEffect, useState } from "react";
import { Pause, Play, RefreshCcw, SkipForward } from "react-feather";

const rs = RestService.getInstance();

export default function Controller() {
    const [state, setState] = useState<State | null>(rs.getState());
    const [speed, setSpeed] = useState<number | null>(rs.getSpeed());

    const [isSending, setSending] = useState(false);
    const [isSetSpeed, setSetSpeed] = useState(false);

    useEffect(() => {
        const stateProfile = rs.onStateChange(setState);
        const speedProfile = rs.onSpeedChange(setSpeed);

        return () => {
            rs.removeListener(stateProfile);
            rs.removeListener(speedProfile);
        }
    }, []);

    return <div className="flex flex-col gap-4">
        <Skeleton isLoaded={state !== null} className="w-min">
            <div className="flex flex-row items-center justify-start whitespace-nowrap">
                {isSending ?
                    <div className="flex flex-row items-center justify-center w-12 mr-2">
                        <Spinner color="primary" size="sm"/>
                    </div>
                : 
                    <Switch isSelected={state?.control_state !== "uncontrolled"} onValueChange={async () => {
                        setSending(true);
                        await rs.setControl(state!.control_state === "uncontrolled");
                        setSending(false);
                    }}></Switch>
                }
                <span>Enable machine control</span>
                </div>
        </Skeleton>
        <Skeleton isLoaded={state !== null} className="w-min rounded-medium">
            <ButtonGroup>
                <Button isIconOnly color="success" variant="ghost"
                    isDisabled={state?.control_state === "uncontrolled" || !["paused", "idle"].includes(state?.control_state || "")}
                    isLoading={isSending}
                    onClick={() => rs.controlAction(Action.PLAY)}>
                    <Play/>
                </Button>
                <Button isIconOnly color="warning" variant="ghost"
                    isDisabled={["idle", "paused", "uncontrolled"].includes(state?.control_state || "")}
                    isLoading={isSending}
                    onClick={() => rs.controlAction(Action.PAUSE)}>
                    <Pause/>
                </Button>
                <Button isIconOnly color="primary" variant="ghost"
                    isDisabled={state?.control_state === "uncontrolled" || !["paused", "idle"].includes(state?.control_state || "")}
                    isLoading={isSending}
                    onClick={() => rs.controlAction(Action.STEP)}>
                    <SkipForward/>
                </Button>
                <Button isIconOnly color="danger" variant="ghost"
                    isDisabled={"uncontrolled" === state?.control_state}
                    isLoading={isSending}
                    onClick={() => rs.controlAction(Action.RESET)}>
                    <RefreshCcw className="text-danger-500 hover:text-danger-foreground"/>
                </Button>
            </ButtonGroup>
        </Skeleton>
        <Skeleton isLoaded={speed !== null}>
            {isSetSpeed ?
                <div className="h-12 flex flex-row items-center">
                    <Progress size="sm" isIndeterminate className="max-w-[500px]"/>
                </div>
            :
                <Slider label="Speed" step={1} maxValue={100} minValue={1} defaultValue={100} 
                    className="max-w-[500px]" value={speed || 0}
                    onChange={e => setSpeed(typeof e === "number" ? e : e[0])}
                    onChangeEnd={async (e) => {
                        setSetSpeed(true);
                        await rs.setSpeed(typeof e === "number" ? e : e[0]);
                        setSetSpeed(false);
                    }}
                />
            }
        </Skeleton>
    </div>
}