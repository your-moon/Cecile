import Playground from "../components/playground";

export default async function PlaygroundPage() {
  return (
    <div className="bg-sky-300 h-screen w-screen">
      <Playground className="h-1/2"></Playground>
    </div>
  );
}
