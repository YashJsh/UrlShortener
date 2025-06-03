import ShortUrl from "@/components/ShortUrl";


export default function Home() {
  return (
    <div className="flex flex-col items-center justify-center w-screen">
        <h1 className="font-semibold items-center py-3">URL Shortener</h1>
        <ShortUrl/>
    </div>
  );
}
