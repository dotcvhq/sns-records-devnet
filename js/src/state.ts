import { deserialize, Schema } from "borsh";
import { Connection, PublicKey } from "@solana/web3.js";
import { Buffer } from "buffer";

export const NAME_REGISTRY_LEN = 96;

export enum Validation {
  None,
  Solana,
  Ethereum,
  UnverifiedSolana,
}

export const getValidationLength = (validation: Validation) => {
  switch (validation) {
    case Validation.None:
      return 0;
    case Validation.Ethereum:
      return 20;
    case Validation.Solana:
      return 32;
    case Validation.UnverifiedSolana:
      return 32;
    default:
      throw new Error("Invalid validation enum");
  }
};

export class RecordHeader {
  stalenessValidation: number;
  rightOfAssociationValidation: number;
  contentLength: number;

  static LEN = 8;

  static schema: Schema = {
    struct: {
      stalenessValidation: "u16",
      rightOfAssociationValidation: "u16",
      contentLength: "u32",
    },
  };

  constructor(obj: {
    stalenessValidation: number;
    rightOfAssociationValidation: number;
    contentLength: number;
  }) {
    this.stalenessValidation = obj.stalenessValidation;
    this.rightOfAssociationValidation = obj.rightOfAssociationValidation;
    this.contentLength = obj.contentLength;
  }

  static deserialize(data: Buffer): RecordHeader {
    return new RecordHeader(deserialize(this.schema, data, true) as any);
  }

  static async retrieve(
    connection: Connection,
    key: PublicKey
  ): Promise<RecordHeader> {
    const info = await connection.getAccountInfo(key);
    if (!info || !info.data) {
      throw new Error("Record header account not found");
    }
    return this.deserialize(
      info.data.slice(NAME_REGISTRY_LEN, NAME_REGISTRY_LEN + this.LEN)
    );
  }
}

export class Record {
  header: RecordHeader;
  data: Buffer;

  constructor(header: RecordHeader, data: Buffer) {
    this.data = data;
    this.header = header;
  }

  static deserialize(buffer: Buffer): Record {
    let offset = NAME_REGISTRY_LEN;
    const header = RecordHeader.deserialize(
      buffer.slice(offset, offset + RecordHeader.LEN)
    );
    const data = buffer.slice(offset + RecordHeader.LEN);
    return new Record(header, data);
  }

  static async retrieve(
    connection: Connection,
    key: PublicKey
  ): Promise<Record> {
    const info = await connection.getAccountInfo(key);
    if (!info || !info.data) {
      throw new Error("Record header account not found");
    }
    return this.deserialize(info.data);
  }

  static async retrieveBatch(
    connection: Connection,
    keys: PublicKey[]
  ): Promise<(Record | undefined)[]> {
    const infos = await connection.getMultipleAccountsInfo(keys);
    const result = infos.map((info) => {
      if (info?.data) {
        return this.deserialize(info.data);
      }
    });
    return result;
  }

  getContent(): Buffer {
    let startOffset =
      getValidationLength(this.header.stalenessValidation) +
      getValidationLength(this.header.rightOfAssociationValidation);
    return this.data.slice(startOffset);
  }

  getStalenessId(): Buffer {
    let endOffset = getValidationLength(this.header.stalenessValidation);
    return this.data.slice(0, endOffset);
  }

  getRoAId(): Buffer {
    let startOffset = getValidationLength(this.header.stalenessValidation);
    let endOffset =
      startOffset +
      getValidationLength(this.header.rightOfAssociationValidation);
    return this.data.slice(startOffset, endOffset);
  }
}
