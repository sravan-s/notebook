import type { SupportedLanguages } from "../../types"

import "./notebook-card.css"

type Props = {
  id: string,
  title: string,
  description: string,
  language: SupportedLanguages,
  createdAt: string,
}

export default function NotebookCard({
  id,
  title,
  description,
  language,
  createdAt,
}: Props) {
  return (
    <article className="notebook-card__wrap">
      <span className="icon">
        {
          language == 'JavaScript'
            ? <JSIcon />
            : <UnknownIcon />
        }
      </span>
      <section>
        <header>
          <a href={`/notebook/${id}`} title={title}>
            {title}
          </a>
        </header>
        <span className="date">{createdAt}</span>
        <p>
          {description}
        </p>
      </section>
    </article>
  )
}

function JSIcon() {
  return <svg xmlns="http://www.w3.org/2000/svg" x="0px" y="0px" width="100" height="100" viewBox="0 0 48 48">
  <path fill="#ffd600" d="M6,42V6h36v36H6z"></path><path fill="#000001" d="M29.538 32.947c.692 1.124 1.444 2.201 3.037 2.201 1.338 0 2.04-.665 2.04-1.585 0-1.101-.726-1.492-2.198-2.133l-.807-.344c-2.329-.988-3.878-2.226-3.878-4.841 0-2.41 1.845-4.244 4.728-4.244 2.053 0 3.528.711 4.592 2.573l-2.514 1.607c-.553-.988-1.151-1.377-2.078-1.377-.946 0-1.545.597-1.545 1.377 0 .964.6 1.354 1.985 1.951l.807.344C36.452 29.645 38 30.839 38 33.523 38 36.415 35.716 38 32.65 38c-2.999 0-4.702-1.505-5.65-3.368L29.538 32.947zM17.952 33.029c.506.906 1.275 1.603 2.381 1.603 1.058 0 1.667-.418 1.667-2.043V22h3.333v11.101c0 3.367-1.953 4.899-4.805 4.899-2.577 0-4.437-1.746-5.195-3.368L17.952 33.029z"></path>
  </svg>
}

function UnknownIcon() {
  return <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 48 48" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
    <circle cx="50%" cy="50%" r="10" stroke="#000" fill="#ffd600"/>
    <path d="M9 9a3 3 0 0 1 6 0c0 3-3 3-3 6" stroke="#ffd600" />
    <line x1="12" y1="17" x2="12" y2="17" stroke="#ffd600" />
  </svg>
}