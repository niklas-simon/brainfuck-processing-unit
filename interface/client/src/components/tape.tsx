import { ChevronDown } from "react-feather";
import Cell from "./cell";
import {Divider} from "@nextui-org/divider";

export default function Tape({data, className, title}: {data: ({address: number, value: string | number} | null)[], className?: string, title: string}) {
    return <div className={className}>
        <div className="flex flex-col items-center w-[232px] overflow-visible">
            <div className="flex flex-row w-full justify-center relative">
                <span className="absolute bottom-1 left-0 text-foreground-500 text-tiny">{title}</span>
                <ChevronDown/>
            </div>
            <div className="flex flex-row w-[454px] justify-end h-[64px]" style={{
                maskImage: "linear-gradient(to right, transparent 32px, black 96px, black 60%, transparent 60%), linear-gradient(to left, transparent 32px, black 96px, black 60%, transparent 60%)"
            }}>
                {data.length ? data.map((d, i) => d ? <div className="flex flex-row" key={i}>
                    {i ? <Divider orientation="vertical" className="h-[64px]"/> : null}
                    <Cell address={d.address} value={d.value}/>
                </div> : <div className="w-[65px] h-[64px]" key={i}></div>) : <div className="w-full flex flex-row justify-center items-center"><span>(empty)</span></div>}
            </div>
        </div>
    </div>
}