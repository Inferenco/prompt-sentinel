import { expect, test } from "@playwright/test";

type ViewportCase = {
  name: string;
  width: number;
  height: number;
};

const viewports: ViewportCase[] = [
  { name: "mobile-portrait", width: 390, height: 844 },
  { name: "mobile-landscape", width: 844, height: 390 },
  { name: "tablet", width: 768, height: 1024 },
  { name: "desktop", width: 1366, height: 768 },
  { name: "desktop-wide", width: 1920, height: 1080 },
];

for (const viewport of viewports) {
  test(`completes a parity flow on ${viewport.name}`, async ({ page }) => {
    await page.setViewportSize({ width: viewport.width, height: viewport.height });
    await page.goto("/");

    await page.getByLabel("Mock mode").check();
    await page.getByTestId("scenario-bias").click();

    const desktopRunButton = page.getByTestId("desktop-run-check-btn");
    if (await desktopRunButton.isVisible()) {
      await desktopRunButton.click();
    } else {
      await page.getByTestId("mobile-run-check-btn").click();
    }

    await expect(page.getByTestId("summary-status")).toContainText("Completed");
    await expect(page.getByTestId("stage-firewall")).toBeVisible();
    await expect(page.getByTestId("stage-bias")).toBeVisible();
    await expect(page.getByTestId("audit-proof")).toBeVisible();
    await expect(page.getByTestId("raw-json-panel")).toBeVisible();
  });
}
