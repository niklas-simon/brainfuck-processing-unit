import { ReactNode } from "react";
import PanelHeader from "./panel-header";
import { Card, CardHeader, CardBody } from "@nextui-org/card";
import { Link } from "@nextui-org/link";

export default function Panel({name, description, icon, children, className, link}: {name: string, description?: string, icon?: ReactNode, children: string | ReactNode | ReactNode[], className?: string, link: string}) {
    return <Card radius="none" className={className}>
        <CardHeader>
            <Link className="lg:hidden" color="foreground" href={link}>
                <PanelHeader icon={icon} name={name} description={description}/>
            </Link>
            <PanelHeader className="hidden lg:block" icon={icon} name={name} description={description}/>
        </CardHeader>
        <CardBody className="hidden lg:flex flex-col">
            {children}
        </CardBody>
    </Card>
}