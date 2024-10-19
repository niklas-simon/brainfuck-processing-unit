import Cell from "./cell";
import {Divider} from "@nextui-org/divider";

export default function Tape({data, className}: {data: {address: number, value: string | number}[], className?: string}) {
    return <div className={className}> 
        <div className="flex flex-row w-[454px] justify-end" style={{
            maskImage: "linear-gradient(to right, transparent 32px, black 96px, black 60%, transparent 60%), linear-gradient(to left, transparent 32px, black 96px, black 60%, transparent 60%)"
        }}>
            {data.map((d) => <div className="flex flex-row" key={d.address}>
                {d.address ? <Divider orientation="vertical" className="h-[64px]"/> : null}
                <Cell address={d.address} value={d.value}/>
            </div>)}
        </div>
    </div>
}