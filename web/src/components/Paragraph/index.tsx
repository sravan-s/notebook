type Props = {}

export default function index({}: Props) {
  return (
    <div className="paragraph">
      <section className="paragrpah-toolbar">
        run, delete, show/hide code
      </section>
      <section className="paragraph-ide">Ide</section>
      <section className="paragraph-result">render result here</section>
    </div>
  )
}