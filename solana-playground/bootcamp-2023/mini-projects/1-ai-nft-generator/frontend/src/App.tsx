import { Input } from "@/components/input";
import { Button } from "@/components/button";
import { Section } from "@/components/section";
import { SparklesIcon } from "lucide-react";

function App() {
  return (
    <div className="h-screen flex flex-col text-[#0D1B2A]">
      <div className="grow flex items-center justify-center">
        <Section>
          <div className="flex gap-2">
            <Input
              type="text"
              placeholder="Enter a prompt to generate an image"
              className="grow"
            />
            <Button className="flex items-center gap-1">
              <SparklesIcon className="size-5" />
              Generate
            </Button>
          </div>
        </Section>
      </div>
    </div>
  );
}

export default App;
