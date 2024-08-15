import "./create-notebook-card.css"

type Props = {}

export default function index({}: Props) {
  return (
    <a className="create-notebook-card" href="/notebooks/create">
      Create Notebook
    </a>
  )
}