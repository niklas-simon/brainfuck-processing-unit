export default function Cell({address, value}: {address: number, value: string | number}) {
    return <div className="bg-content3 flex flex-col items-center justify-center relative p-4 w-[64px] h-[64px]">
        <span className="absolute top-1 left-1 text-xs font-mono">{address.toString(16)}</span>
        <span className="text-2xl font-mono">{typeof value === "string" ? value : value.toString(16)}</span>
    </div>
}