import RestService, { Example } from "@/run-service";
import { Button } from "@nextui-org/button";
import { Dropdown, DropdownItem, DropdownMenu, DropdownTrigger } from "@nextui-org/dropdown";
import { Textarea } from "@nextui-org/input";
import { Skeleton } from "@nextui-org/skeleton";
import { useEffect, useState } from "react";

const rs = RestService.getInstance();
const presetRequest = rs.getExamples();

export default function Programmer() {
    const [program, setProgram] = useState<string | null>(rs.getProgram());
    const [presets, setPresets] = useState<Example[] | null>(null);

    const [isWriting, setWriting] = useState(false);
    const [selectedPreset, setSelectedPreset] = useState(new Set<string>());

    useEffect(() => {
        if (presets === null) {
            presetRequest.then(setPresets);
        }

        const programProfile = rs.onProgramChange(setProgram);

        return () => {
            rs.removeListener(programProfile);
        }
    }, []);

    useEffect(() => {
        const selected = presets?.map((p, i) => Object.assign(p, {i})).filter(p => p.code === program).map(p => p.i.toString()) || [];
        setSelectedPreset(new Set(selected));
    }, [presets, program]);

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
                    {presets && <DropdownMenu onAction={key => {
                        setProgram(presets[key as number].code);
                    }} selectionMode="single" selectedKeys={selectedPreset} disallowEmptySelection>
                        {presets.map((p, i) => <DropdownItem key={i.toString()} title={p.name} description={p.desc}/>)}
                    </DropdownMenu>}
                </Dropdown>
            </Skeleton>
        </div>
    </div>
}