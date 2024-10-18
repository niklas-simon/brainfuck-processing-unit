import { ReactNode } from "react";

export default function PanelHeader({icon, name, description, className}: {icon: ReactNode, name: string, description?: string, className?: string}) {
    return <div className={className}>
        <div className="flex flex-row items-center gap-4">
            {icon}
            <div className="flex flex-col justify-center">
                <p className="text-md">{name}</p>
                {description && <p className="text-small text-default-500">{description}</p>}
            </div>
        </div>
    </div>
}