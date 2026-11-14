use pumpkin_data::packet::clientbound::play::TAB_LIST;
use pumpkin_util::text::TextComponent;

use crate::ClientPacket;
use crate::packet::MultiVersionJavaPacket;
use crate::ser::NetworkWriteExt;
use pumpkin_util::version::JavaMinecraftVersion;

pub struct CTabList<'a> {
    pub header: &'a TextComponent,
    pub footer: &'a TextComponent,
}

impl<'a> CTabList<'a> {
    #[must_use]
    pub const fn new(header: &'a TextComponent, footer: &'a TextComponent) -> Self {
        Self { header, footer }
    }
}

impl MultiVersionJavaPacket for CTabList<'_> {
    fn to_id(version: JavaMinecraftVersion) -> i32 {
        if version >= JavaMinecraftVersion::V_26_1 {
            TAB_LIST.to_id(version)
        } else if version >= JavaMinecraftVersion::V_1_21_2 {
            0x6E
        } else if version >= JavaMinecraftVersion::V_1_20_5 {
            0x6E
        } else if version >= JavaMinecraftVersion::V_1_20_2 {
            0x6C
        } else {
            0x62
        }
    }
}

impl ClientPacket for CTabList<'_> {
    fn write_packet_data(
        &self,
        mut write: impl std::io::Write,
        version: &JavaMinecraftVersion,
    ) -> Result<(), crate::ser::WritingError> {
        write.write_component(self.header, version)?;
        write.write_component(self.footer, version)?;
        Ok(())
    }
}
