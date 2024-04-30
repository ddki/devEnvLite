const disableRefresh = () => {
	document.addEventListener("keydown", (event) => {
		if (
			event.key === "F5" ||
			(event.ctrlKey && event.key.toLowerCase() === "r") ||
			(event.metaKey && event.key.toLowerCase() === "r")
		) {
			event.preventDefault();
		}
		// if (
		// 	event.key === "F12" ||
		// 	(event.ctrlKey && event.shiftKey && event.key.toLowerCase() === "i") ||
		// 	(event.metaKey && event.shiftKey && event.key.toLowerCase() === "i")
		// ) {
		// 	event.preventDefault();
		// }
	});
};

const disableContextMenu = () => {
	document.addEventListener("contextmenu", (event) => {
		event.preventDefault();
	});
};

export { disableRefresh, disableContextMenu };
