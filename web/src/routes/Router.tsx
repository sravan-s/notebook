import {
  createBrowserRouter,
  createRoutesFromElements,
  Route,
} from "react-router-dom";
import Notebooks from "./Notebooks";
import Notebook from "./Notebook";
import CreateNotebook from "./CreateNotebook";
import NotebookView from "./NotebookView";
import NotebookSettings from "./NotebookSettings";

export const ROUTES = [
  {
    path: "/",
    element: <Notebooks />,
  },
  {
    path: "/notebook/:notebookId",
    element: <Notebook />,
  },
  {
    path: "/notebooks/create",
    element: <CreateNotebook />,
  },
  {
    path: "/notebook/:notebookId/view",
    element: <NotebookView />,
  },
  {
    path: "/notebook/:notebookId/settings",
    element: <NotebookSettings />,
  },
  // maybe add paragraph?
];

export const router = createBrowserRouter(
  createRoutesFromElements(
    ROUTES.map((route) => (
      <Route key={route.path} path={route.path} element={route.element} />
    ))
  )
);
