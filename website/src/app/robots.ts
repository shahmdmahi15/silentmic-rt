import { MetadataRoute } from "next";

export default function robots(): MetadataRoute.Robots {
  return {
    rules: {
      userAgent: "*",
      allow: "/",
    },
    sitemap: "https://silentmic-rt.shahmdmahi.dpdns.org/sitemap.xml",
  };
}
