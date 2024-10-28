import { Table, TableBody, TableCell, TableColumn, TableHeader, TableRow } from "@nextui-org/table";
import Tape from "./tape";
import { Card, CardBody } from "@nextui-org/card";
import { Chip } from "@nextui-org/chip";
import { useEffect, useState } from "react";
import RestService, { State } from "@/run-service";
import { Skeleton } from "@nextui-org/skeleton";

const states = {
    running: {
        text: "Running",
        color: "success"
    },
    startup: {
        text: "Starting",
        color: "warning"
    },
    jumping: {
        text: "Jumping",
        color: "warning"
    },
    wait_input: {
        text: "Waiting for input",
        color: "danger"
    },
    output_ready: {
        text: "Waiting for output",
        color: "danger"
    },
    idle: {
        text: "Finished",
        color: "success"
    }
} as {[key: string]: {text: string, color: "success" | "danger" | "warning"}}

const rs = RestService.getInstance();
const programRequest = rs.getProgram();
const inputRequest = rs.getInput();
const stateRequest = rs.getState();

export default function Twin() {
    const [program, setProgram] = useState<string | null>(null);
    const [input, setInput] = useState<string | null>(null);
    const [state, setState] = useState<State | null>(null);

    useEffect(() => {
        if (program === null) {
            programRequest.then(setProgram);
        }
        if (input === null) {
            inputRequest.then(setInput);
        }
        if (state === null) {
            stateRequest.then(setState);
        }

        const programEvent = rs.getProgramEvent();
        const onProgramMessage = (e: MessageEvent) => {
            setProgram(e.data);
        }
        programEvent.addEventListener("message", onProgramMessage);

        const inputEvent = rs.getInputEvent();
        const onInputMessage = (e: MessageEvent) => {
            setInput(e.data);
        }
        inputEvent.addEventListener("message", onInputMessage);
        
        const stateEvent = rs.getProgramEvent();
        const onStateMessage = (e: MessageEvent) => {
            console.log(e);
            setState(JSON.parse(e.data) as State);
        }
        console.log("add event listener");
        stateEvent.addEventListener("message", onStateMessage);
        
        return () => {
            console.log("remove event listener");
            programEvent.removeEventListener("message", onProgramMessage);
            inputEvent.removeEventListener("message", onInputMessage);
            stateEvent.removeEventListener("message", onStateMessage);
        }
    }, []);

    return <div className="w-full h-full flex flex-col items-center justify-center overflow-hidden">
        {["idle", "uncontrolled"].includes(state?.control || "") ?
            <Chip color="warning" variant="flat">currently not controlled or idle</Chip>
        :
            <Skeleton isLoaded={state !== null}>
                <Card className="bg-content2 overflow-visible" radius="none">
                    <CardBody className="flex flex-col items-center overflow-visible gap-4">
                        <Tape title="Program" data={Array.from(new Array(7)).map((_, i) => {
                            if (!state || !program) {
                                return null;
                            }
                            if (state.code!.pc - 3 + i < 0) {
                                return null;
                            }
                            if (state.code!.pc - 3 + i >= program.length) {
                                return null;
                            }
                            return {
                                address: state.code!.pc - 3 + i,
                                value: program[state.code!.pc - 3 + i]
                            }
                        })}/>
                        <Tape title="Memory" data={state?.tape!.map((v, i) => {
                            return {
                                address: state.head! - 3 + i,
                                value: v
                            }
                        }) || []}/>
                        <div className="flex flex-row gap-2 justify-start w-full">
                            <Table radius="none" isCompact={true} fullWidth={false} removeWrapper={true} className="bg-content1 w-min"> 
                                <TableHeader>
                                    <TableColumn className="bg-content1 rounded-none h-min pt-2">Stack</TableColumn>
                                </TableHeader>
                                <TableBody>
                                    {state?.stack!.length ? 
                                        state.stack.slice(0, 6).map((v, i) => {
                                            return <TableRow key={i}>
                                                <TableCell className="font-mono">
                                                    {state.stack!.length > 6 && i === 5 ? "..." : "0x" + v.toString(16).padStart(4, "0")}
                                                </TableCell>
                                            </TableRow>
                                        })
                                    :
                                        <TableRow>
                                            <TableCell>
                                                (empty)
                                            </TableCell>
                                        </TableRow>
                                    }
                                </TableBody>
                            </Table>
                            <div className="flex flex-col gap-2">
                                {Object.keys(states).map((s, i) => {
                                    return <Chip key={i} variant="dot" className="border-none" color={state?.control === s ? states[s].color : "default"}>{states[s].text}</Chip>
                                })}
                            </div>
                        </div>
                        <div className="flex flex-col w-[232px] overflow-hidden gap-1" style={{
                            maskImage: "linear-gradient(to left, transparent, black 64px)"
                        }}>
                            <span className="text-foreground-500 text-tiny">Input queue</span>
                            <div className="flex flex-row gap-2 font-mono w-max">
                                {input ? input.split("").slice(state?.ic).map((v, i) => <div key={i} className="p-1 bg-content1 w-[1.5em] text-center">{v}</div>) : <span>(empty)</span>}
                            </div>
                        </div>
                    </CardBody>
                </Card>
            </Skeleton>
        }
    </div>
}