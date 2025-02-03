"use client";

import { useState } from "react";
import { Button } from "@/components/ui/button";

export default function Home() {
  const [isLoading, setIsLoading] = useState(false);

  const handleClick = () => {
    setIsLoading(true);
    setTimeout(() => {
      setIsLoading(false);
    }, 2000);
  };
  return (
    <div className="flex justify-center items-center min-h-screen">
      <Button isLoading={isLoading} onClick={handleClick}>
        Click me
      </Button>
    </div>
  );
}
