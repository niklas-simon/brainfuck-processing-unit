import { Table, TableBody, TableCell, TableColumn, TableHeader, TableRow } from "@nextui-org/table";
import Tape from "./tape";
import { Card, CardBody } from "@nextui-org/card";
import { Chip } from "@nextui-org/chip";
import { useCallback, useEffect, useState } from "react";

const states = {
    RUNNING: {
        text: "Running",
        color: "success"
    },
    STARTING: {
        text: "Starting",
        color: "warning"
    },
    JUMPING: {
        text: "Jumping",
        color: "warning"
    },
    INPUT: {
        text: "Waiting for input",
        color: "danger"
    },
    OUTPUT: {
        text: "Waiting for output",
        color: "danger"
    },
    FINISHED: {
        text: "Finished",
        color: "success"
    }
} as {[key: string]: {text: string, color: "success" | "danger" | "warning"}}

export default function Twin() {
    const [program, setProgram] = useState(window.localStorage.getItem("program") || "");
    const [input, setInput] = useState("");
    const pc = Math.floor(Math.random() * program.length);
    const pointer = Math.floor(Math.random() * 0x10000);

    const handleProgramChange = useCallback((e: Event) => {
        setProgram((e as CustomEvent<string>).detail);
    }, []);

    const handleInputChange = useCallback((e: Event) => {
        setInput(input + (e as CustomEvent<string>).detail);
    }, [input]);

    useEffect(() => {
        window.addEventListener("program", handleProgramChange);
        window.addEventListener("sendInput", handleInputChange);
        return () => {
            window.removeEventListener("program", handleProgramChange);
            window.removeEventListener("sendInput", handleInputChange)
        }
    }, [handleProgramChange, handleInputChange]);

    const state = {
        pc,
        pointer,
        data: Array.from(new Array(7)).map((_, i) => {
            return {
                address: (0x10000 + i + pointer - 3) % 0x10000,
                value: Math.floor(Math.random() * 0x100)
            }
        }),
        stack: Array.from(new Array(Math.floor(Math.random() * 8))).map(() => {
            return Math.floor(Math.random() * 0x10000);
        }).sort(),
        state: Object.keys(states)[Math.floor(Math.random() * Object.keys(states).length)],
        input
    }

    return <div className="w-full h-full flex flex-col items-center justify-center overflow-hidden">
        <Card className="bg-content2 overflow-visible" radius="none">
            <CardBody className="flex flex-col items-center overflow-visible gap-4">
                <Tape title="Program" data={Array.from(new Array(7)).map((_, i) => {
                    if (state.pc - 3 + i < 0) {
                        return null;
                    }
                    if (state.pc - 3 + i >= program.length) {
                        return null;
                    }
                    return {
                        address: state.pc - 3 + i,
                        value: program[state.pc - 3 + i]
                    }
                })}/>
                <Tape title="Memory" data={state.data}/>
                <div className="flex flex-row gap-2 justify-start w-full">
                    <Table radius="none" isCompact={true} fullWidth={false} removeWrapper={true} className="bg-content1 w-min"> 
                        <TableHeader>
                            <TableColumn className="bg-content1 rounded-none h-min pt-2">Stack</TableColumn>
                        </TableHeader>
                        <TableBody>
                            {state.stack.length ? 
                                state.stack.slice(0, 6).map((v, i) => {
                                    return <TableRow key={i}>
                                        <TableCell className="font-mono">
                                            {state.stack.length > 6 && i === 5 ? "..." : "0x" + v.toString(16).padStart(4, "0")}
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
                            return <Chip key={i} variant="dot" className="border-none" color={state.state === s ? states[s].color : "default"}>{states[s].text}</Chip>
                        })}
                    </div>
                </div>
                <div className="flex flex-col w-[232px] overflow-hidden gap-1" style={{
                    maskImage: "linear-gradient(to left, transparent, black 64px)"
                }}>
                    <span className="text-foreground-500 text-tiny">Input queue</span>
                    <div className="flex flex-row gap-2 font-mono w-max">
                        {state.input.length ? state.input.split("").map((v, i) => <div key={i} className="p-1 bg-content1 w-[1.5em] text-center">{v}</div>) : <span>(empty)</span>}
                    </div>
                </div>
            </CardBody>
        </Card>
    </div>
}