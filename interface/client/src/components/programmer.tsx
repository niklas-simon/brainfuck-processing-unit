import RestService, { Example } from "@/run-service";
import { Button } from "@nextui-org/button";
import { Dropdown, DropdownItem, DropdownMenu, DropdownTrigger } from "@nextui-org/dropdown";
import { Textarea } from "@nextui-org/input";
import { Skeleton } from "@nextui-org/skeleton";
import { useEffect, useState } from "react";

const rs = RestService.getInstance();
const presetRequest = rs.getExamples();
const programRequest = rs.getProgram();

export default function Programmer() {
    const [program, setProgram] = useState<string | null>(null);
    const [presets, setPresets] = useState<Example[] | null>(null);
    const [isWriting, setWriting] = useState(false);

    useEffect(() => {
        if (presets === null) {
            presetRequest.then(setPresets);
        }
        if (program === null) {
            programRequest.then(setProgram);
        }

        const programEvent = rs.getProgramEvent();
        const onProgramMessage = (e: MessageEvent) => {
            console.log(e);
            setProgram(e.data);
        }
        programEvent.addEventListener("message", onProgramMessage);

        return () => {
            programEvent.removeEventListener("message", onProgramMessage);
        }
    }, []);

    const writeProgram = async () => {
        setWriting(true);
        await rs.setProgram(program!);
        setWriting(false);
    };

    return <div className="flex flex-col gap-4">
        <Skeleton isLoaded={program !== null}>
            <Textarea label="Program" placeholder=",[.,]" radius="none" className="font-mono"
                value={program || ""} onChange={e => setProgram(e.target.value)}/>
        </Skeleton>
        <div className="flex flex-row gap-4">
            <Skeleton isLoaded={program !== null} className="rounded-medium">
                <Button color="primary" variant="light" onClick={writeProgram} isLoading={isWriting}>Write</Button>
            </Skeleton>
            <Skeleton isLoaded={program !== null && presets !== null} className="rounded-medium">
                <Dropdown>
                    <DropdownTrigger>
                        <Button variant="light" color="secondary">
                            Preset
                        </Button>
                    </DropdownTrigger>
                    {presets && <DropdownMenu onAction={key => setProgram(presets[key as number].code)}>
                        {presets.map((p, i) => <DropdownItem key={i}>
                            {p.name}
                        </DropdownItem>)}
                    </DropdownMenu>}
                </Dropdown>
            </Skeleton>
        </div>
    </div>
}