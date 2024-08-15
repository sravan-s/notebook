import Paragraph from "../Paragraph"

type Props = {}

export default function index({}: Props) {

  // get notebook data
  return (
    <article className="notebook">
      <header>
        <h1>Notebook title</h1>
        <span>date</span>
        <button>Run All</button>
        <button>settings</button>
      </header>
      <section className="paragraphs">
        {/* masonary -> Initially, no resize */}
        <Paragraph />
        <Paragraph />
        <Paragraph />
        <Paragraph />
      </section>
    </article>
  )
}