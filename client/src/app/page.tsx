"use client";

export default function Home() {
  const uploadHandler = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file: File | undefined = e.target.files?.[0];
    if (!file) return;
    console.log(file);
    const formData: FormData = new FormData();
    formData.append("file", file);
    const result = fetch("http://localhost:5000/", {
      method: "POST",
      body: formData,
    });
  };
  return (
    <form>
      <input type="file" accept="image/*" onChange={(e) => uploadHandler(e)} />
    </form>
  );
}
