import { A } from "solid-start";

export default function NotFound() {
    return (
        <div class="flex m-auto container max-w-lg">
            <h1>Member not found</h1>
            <A href="/" class="btn">
                Go home
            </A>
        </div>
    );
}
