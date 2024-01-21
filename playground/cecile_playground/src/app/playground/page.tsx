import Playground from "../components/playground";

export default async function PlaygroundPage() {
  return (
    <div className="bg-black h-screen w-screen">
      <Playground className="h-1/2"></Playground>
    </div>
  );
}
