import Controller from "@/components/controller";
import IO from "@/components/io";
import Panel from "@/components/panel";
import Programmer from "@/components/programmer";
import Twin from "@/components/twin";
import DefaultLayout from "@/layouts/default";
import { Activity, Code, Sliders, Terminal } from "react-feather";

export default function IndexPage() {
    return (
        <DefaultLayout>
            <div className="flex flex-col lg:grid lg:grid-cols-5 lg:auto-rows-min gap-4">
                <Panel name="Programmer" description="Write programs to the connected machine" icon={<Code/>} className="row-start-1 row-end-2 col-span-full" link="/programmer">
                    <Programmer/>
                </Panel>
                <Panel name="I/O" description="Send input to or display output from the connected machine" icon={<Terminal/>} className="row-start-2 row-end-3 col-span-2" link="/io">
                    <IO />
                </Panel>
                <Panel name="Controller" description="Take control over the connected machine" icon={<Sliders/>} className="row-start-3 row-end-4 col-span-2" link="/controller">
                    <Controller />
                </Panel>
                <Panel name="Digital Twin" description="View the connected machine's state" icon={<Activity/>} className="row-start-2 row-end-4 col-start-3 col-end-6" link="/twin">
                    <Twin />
                </Panel>
            </div>
        </DefaultLayout>
    );
}
