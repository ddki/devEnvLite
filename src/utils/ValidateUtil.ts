const validateUrl = (url: string) => {
	const urlRegex = new RegExp(
		"^(https?:\\/\\/)?" + // 协议 http/https
			"(([a-zA-Z\\d]([a-zA-Z\\d-]*[a-zA-Z\\d])*)\\.)+" + // 域名
			"([a-zA-Z]{2,}|[0-9]{1,3})(\\:\\d+)?" + // 顶级域 .com / 端口号
			"(\\/[-a-zA-Z\\d%_.~+]*)*" + // 路径
			"(\\?[;&a-zA-Z\\d%_.~+=-]*)?" + // 查询参数
			"(\\#[-a-zA-Z\\d_]*)?$", // 锚点
	);
	return urlRegex.test(url);
};

export { validateUrl };
