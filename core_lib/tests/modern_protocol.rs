use std::collections::HashMap;

use prost::Message;
use rqs_lib::sharing_nearby::{
    attachment_details, file_metadata, introduction_frame, AttachmentDetails,
    ConnectionResponseFrame, FileAttachmentDetails, FileMetadata, IntroductionFrame,
    PairedKeyEncryptionFrame, PayloadDetails, PayloadsDetails, StreamMetadata,
};

#[test]
fn modern_introduction_fields_round_trip() {
    let frame = IntroductionFrame {
        file_metadata: vec![FileMetadata {
            name: Some("photo.jpg".to_owned()),
            r#type: Some(file_metadata::Type::Image.into()),
            payload_id: Some(1001),
            size: Some(42),
            mime_type: Some("image/jpeg".to_owned()),
            id: Some(5001),
            parent_folder: Some("Trip/Day 1".to_owned()),
            attachment_hash: Some(0x11223344),
            is_sensitive_content: Some(false),
        }],
        stream_metadata: vec![StreamMetadata {
            description: Some("live stream".to_owned()),
            package_name: Some("com.example.app".to_owned()),
            payload_id: Some(2001),
            attributed_app_name: Some("Example".to_owned()),
        }],
        use_case: Some(introduction_frame::SharingUseCase::RemoteCopy.into()),
        preview_payload_ids: vec![3001, 3002],
        transfer_id: Some("transfer-123".to_owned()),
        start_transfer: Some(true),
        ..Default::default()
    };

    let encoded = frame.encode_to_vec();
    let decoded = IntroductionFrame::decode(encoded.as_slice()).unwrap();

    let file = &decoded.file_metadata[0];
    assert_eq!(file.parent_folder.as_deref(), Some("Trip/Day 1"));
    assert_eq!(file.attachment_hash, Some(0x11223344));
    assert_eq!(
        decoded.use_case(),
        introduction_frame::SharingUseCase::RemoteCopy
    );
    assert_eq!(decoded.preview_payload_ids, vec![3001, 3002]);
    assert_eq!(decoded.transfer_id.as_deref(), Some("transfer-123"));
    assert!(decoded.start_transfer());
    assert_eq!(decoded.stream_metadata[0].payload_id, Some(2001));
}

#[test]
fn qr_handshake_material_round_trips() {
    let frame = PairedKeyEncryptionFrame {
        signed_data: Some(vec![1, 2, 3]),
        secret_id_hash: Some(vec![4, 5]),
        optional_signed_data: Some(vec![6]),
        qr_code_handshake_data: Some(vec![7, 8, 9, 10]),
    };

    let encoded = frame.encode_to_vec();
    let decoded = PairedKeyEncryptionFrame::decode(encoded.as_slice()).unwrap();

    assert_eq!(decoded.qr_code_handshake_data, Some(vec![7, 8, 9, 10]));
}

#[test]
fn resume_attachment_details_round_trip() {
    let mut payloads = HashMap::new();
    payloads.insert(
        12345,
        PayloadsDetails {
            payload_details: vec![PayloadDetails {
                id: Some(9001),
                creation_timestamp_millis: Some(1_700_000_000_000),
                size: Some(4096),
            }],
        },
    );

    let mut details = HashMap::new();
    details.insert(
        67890,
        AttachmentDetails {
            r#type: Some(attachment_details::Type::File.into()),
            file_attachment_details: Some(FileAttachmentDetails {
                receiver_existing_file_size: Some(2048),
                attachment_hash_payloads: payloads,
            }),
        },
    );

    let frame = ConnectionResponseFrame {
        status: Some(rqs_lib::sharing_nearby::connection_response_frame::Status::Accept.into()),
        attachment_details: details,
        stream_metadata: vec![],
    };

    let encoded = frame.encode_to_vec();
    let decoded = ConnectionResponseFrame::decode(encoded.as_slice()).unwrap();

    let details = decoded.attachment_details.get(&67890).unwrap();
    let file = details.file_attachment_details.as_ref().unwrap();
    assert_eq!(file.receiver_existing_file_size, Some(2048));
    assert_eq!(
        file.attachment_hash_payloads[&12345].payload_details[0].size,
        Some(4096)
    );
}

#[test]
fn legacy_introduction_defaults_remain_compatible() {
    let legacy = IntroductionFrame {
        file_metadata: vec![FileMetadata {
            name: Some("legacy.txt".to_owned()),
            r#type: Some(file_metadata::Type::Document.into()),
            payload_id: Some(7),
            size: Some(12),
            mime_type: Some("text/plain".to_owned()),
            id: Some(9),
            ..Default::default()
        }],
        ..Default::default()
    };

    let encoded = legacy.encode_to_vec();
    let decoded = IntroductionFrame::decode(encoded.as_slice()).unwrap();

    assert_eq!(decoded.file_metadata.len(), 1);
    let file = &decoded.file_metadata[0];
    assert_eq!(file.name(), "legacy.txt");
    assert_eq!(file.parent_folder, None);
    assert_eq!(file.attachment_hash, None);
    assert_eq!(file.is_sensitive_content, None);
    assert!(decoded.app_metadata.is_empty());
    assert!(decoded.stream_metadata.is_empty());
    assert!(decoded.preview_payload_ids.is_empty());
    assert_eq!(decoded.transfer_id, None);
    assert_eq!(
        decoded.use_case(),
        introduction_frame::SharingUseCase::Unknown
    );
}

