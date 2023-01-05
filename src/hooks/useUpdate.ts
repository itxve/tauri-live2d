import { checkupdate } from "@/plugins";

export default async function () {
  await checkupdate.check_version_update();
}
