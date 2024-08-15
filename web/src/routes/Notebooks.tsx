import { useState } from "react"

import "./notebooks.css"
import NotebookCard from "../components/NotebookCard";
import CreateNotebookCard from "../components/CreateNotebookCard";

type Props = {}

export default function Notebooks({}: Props) {
  const [notebooks, _setNotebooks] = useState<Array<number>>(Array(20).fill(0).map((_m, idx)=> idx));
  return (
    <div className="notebook__wrapper">
      <CreateNotebookCard />
      {
        notebooks.map(n => {
          return <NotebookCard
            key={n}
            id={`${n}`}
            title="title"
            description="desc"
            createdAt="123"
            language="JavaScript"
          />
        })
      }
    </div>
  )
}