# This is free and unencumbered software released into the public domain.

module OpenAPI; end
module OpenAPI::Transforms; end

module OpenAPI::Flattener
  DEFAULT_ARRAY_TYPE = { type: 'integer' }

  ##
  # @param  [Hash] The `openapi.components.schemas` map
  # @return [Hash]
  def flatten_openapi_schemas(input)
    output = {}
    input.each do |schema_ref, schema|
      flatten_definition(schema_ref, schema, output)
    end
    output
  end

  protected

  def flatten_definition(schema_ref, schema, output = {})
    case
      when schema[:'$ref']
        output[schema_ref] = schema
      when schema[:oneOf], schema[:anyOf], schema[:allOf]
        # TODO: allOf
        variants = schema[:oneOf] || schema[:anyOf] || schema[:allOf]
        return flatten_definition(schema_ref, variants.first, output) if variants.size == 1
        variants.map!.with_index do |variant, i|
          variant_ref = [schema_ref, "Variant#{i + 1}"].join('_').to_sym
          flatten_schema(variant_ref, variant, output)
        end
        output[schema_ref] = schema
      when schema[:properties] || schema[:type] == 'object'
        # TODO: additionalProperties
        properties = schema[:properties] || {}
        properties.each do |property_id, property|
          property_ref = [schema_ref, snake_to_camel(property_id)].join('_').to_sym
          properties[property_id] = flatten_schema(property_ref, property, output)
        end
        output[schema_ref] = schema
      when schema[:items] || schema[:type] == 'array'
        schema[:items] = flatten_schema(schema_ref, schema[:items], output)
        output[schema_ref] = schema
      else
        output[schema_ref] = schema
    end
  end

  def flatten_schema(schema_ref, schema, output = {}, level = 1)
    case
      when schema[:$ref]
        schema
      when schema[:oneOf], schema[:anyOf], schema[:allOf]
        # TODO: allOf
        variants = schema[:oneOf] || schema[:anyOf] || schema[:allOf]
        return flatten_definition(schema_ref, variants.first, output) if variants.size == 1
        variants.map!.with_index do |variant, i|
          variant_ref = [schema_ref, "Variant#{i + 1}"].join('_').to_sym
          flatten_schema(variant_ref, variant, output)
        end
        nullable = schema.delete(:nullable)
        output[schema_ref] = schema
        { :'$ref' => "#/components/schemas/#{schema_ref}", nullable: nullable }.compact
        # TODO: description is lost
      when schema[:properties] || schema[:type] == 'object'
        return schema if !schema[:properties]
        properties = schema[:properties]
        properties.each do |property_id, property|
          property_ref = [schema_ref, snake_to_camel(property_id)].join('_').to_sym
          properties[property_id] = flatten_schema(property_ref, property, output, level + 1)
        end
        output[schema_ref] = schema
        { :'$ref' => "#/components/schemas/#{schema_ref}" }
      when schema[:items] || schema[:type] == 'array'
        schema[:items] = schema[:items] ? flatten_schema(schema_ref, schema[:items], output) : DEFAULT_ARRAY_TYPE
        schema
      else
        schema # nothing needed for primitive types
    end
  end

  def snake_to_camel(name)
    name.to_s.gsub(/[_.]/, ' ').split.map(&:capitalize).join
  end
end # OpenAPI::Flattener
